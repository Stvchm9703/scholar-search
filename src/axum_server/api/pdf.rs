use anyhow::Error;
use reqwest;
use std::io::Cursor;
use tracing::warn;

use lopdf;

use fast_symspell::{AsciiStringStrategy, SymSpell};
use std::fs;

pub async fn pdf_download(paper_id: &str, url: &str) -> Result<(), Error> {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_static("Mozilla/5.0"),
    );
    let client = reqwest::Client::builder()
        .default_headers(header)
        .cookie_store(true)
        .build()?;
    let resp = client.get(url).send().await?;

    // check path is exist
    fs::create_dir_all("data/pdf_temp")?;
    let paper_path = paper_id.replace("/", "__").replace(".", "__");
    let mut out = std::fs::File::create(format!("data/pdf_temp/{}.pdf", paper_path))?;
    let content_bytes = resp.bytes().await?;
    let mut content = Cursor::new(content_bytes.to_owned());
    std::io::copy(&mut content, &mut out)?;

    if let Ok(content_str) = String::from_utf8(content_bytes.to_vec()) {
        println!("content_str: {:#?}", content_str);
        if content_str.contains(r#"<head>"#) {
            warn!("not a pdf format: {:?}", paper_id);
            return Err(anyhow::anyhow!("content_str: {:#?}", content_str));
        }
    }
    Ok(())
}

fn collect_text(text: &mut String, encoding: Option<&str>, operands: &[lopdf::Object]) {
    for operand in operands.iter() {
        match *operand {
            lopdf::Object::String(ref bytes, _) => {
                let decoded_text = lopdf::Document::decode_text(encoding, bytes);
                text.push_str(&decoded_text);
            }
            lopdf::Object::Array(ref arr) => {
                collect_text(text, encoding, arr);
                text.push(' ');
            }
            lopdf::Object::Integer(i) => {
                if i < -100 {
                    text.push(' ');
                }
            }
            _ => {}
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ConcatedOperation {
    operator: String,
    content: String,
}

fn decode_operation(
    doc: lopdf::Document,
    page_id: lopdf::ObjectId,
    page_number: u32,
    _symspell: &SymSpell<AsciiStringStrategy>,
) -> String {
    let mut text = String::new();

    let fonts = doc.get_page_fonts(page_id);
    let encodings = fonts
        .into_iter()
        .map(|(name, font)| (name, font.get_font_encoding()))
        .collect::<std::collections::BTreeMap<Vec<u8>, &str>>();
    let content_data = doc.get_page_content(page_id).unwrap();
    let content = lopdf::content::Content::decode(&content_data).unwrap();
    let mut current_encoding = None;

    let content_vec: Vec<ConcatedOperation> = vec![];

    for operation in &content.operations {
        // let mut instant_text = String::from("");
        match operation.operator.as_ref() {
            "Tf" => {
                let current_font = operation
                    .operands
                    .get(0)
                    .ok_or_else(|| lopdf::Error::Syntax("missing font operand".to_string()))
                    .unwrap()
                    .as_name()
                    .unwrap();
                current_encoding = encodings.get(current_font).cloned();
                // instant_text = String::from("");
            }
            "Tj" | "TJ" => {
                collect_text(&mut text, current_encoding, &operation.operands);
            }
            "ET" => {
                if !text.ends_with('\n') {
                    text.push('\n')
                }

                // instant_text = refix
                //     .replace_all(&instant_text, "$noun1 $noun2")
                //     .to_string();
            }
            _ => {}
        }
    }
    if page_number == 1 {
        println!("{:#?}", content_vec);
    }

    text
}

pub async fn convert_pdf_to_text(paper_id: &str) -> Result<(), Error> {
    let paper_wrap = format!(
        "data/pdf_temp/{}.pdf",
        paper_id.replace("/", "__").replace(".", "__")
    );
    let paper_path = std::path::Path::new(&paper_wrap);
    // let text_content = pdf_extract_text( paper_path)?;
    // println!("{}" , text_content);
    let doc = lopdf::Document::load(paper_path)?;
    if let Ok(toc) = doc.get_toc() {
        println!("toc: {:#?}", toc);
    }

    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
    symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    symspell.load_bigram_dictionary(
        "./data/frequency_bigramdictionary_en_243_342.txt",
        0,
        2,
        " ",
    );

    // let pages = doc.get_pages();
    let page = doc.get_pages();
    for (page_number, page_id) in page {
        println!("[page-num]: {}", page_number);
        // println!("{}", content_2);
        // println!("-----------------");
        // let content= doc.extract_text(&vec![page_number]).unwrap();
        let content = decode_operation(doc.clone(), page_id, page_number, &symspell);

        println!("{}", content);
        // content

        // content = symspell.word_segmentation(&content, 3).segmented_string;
        // println!("{:#?}", sugg);
    }
    Ok(())
}

mod test {
    
    use super::*;
    use convert_case::{Case, Casing};
    use fast_symspell::{AsciiStringStrategy, SymSpell};
    use regex::Regex;
    // use speller::Speller;
    
    #[tokio::test]
    async fn test_pdf_download() {
        println!(
            "test_pdf_download {}",
            "10.1145/3292500.3330648".to_case(Case::Snake)
        );
        pdf_download(
            "10.1145/3292500.3330648",
            "https://dl.acm.org/doi/pdf/10.1145/3292500.3330648",
        )
        .await
        .unwrap();
        // convert_pdf_to_text("10.1145/3292500.3330648").await.unwrap();
    }

    #[tokio::test]
    async fn test_convert_pdf_to_text() {
        // convert_pdf_to_text("10.1145/3292500.3330648")
        convert_pdf_to_text("c27ad9346f384e828a4cd6dc8e7e724ea54bd1a2")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_convert_pdf_to_text_1() {
        let paper_id = "c27ad9346f384e828a4cd6dc8e7e724ea54bd1a2";
        let paper_path = paper_id.replace("/", "__").replace(".", "__");
        // let mut out = std::fs::File::create()?;
        // let _yy = pdf_extract_text(format!("data/pdf_temp/{}.pdf", paper_path)).unwrap();
    }

    #[tokio::test]
    async fn test_pdf_extract_text() {
        // let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
        // symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
        // symspell.load_bigram_dictionary(
        //     "./data/frequency_bigramdictionary_en_243_342.txt",
        //     0,
        //     2,
        //     " ",
        // );
        let speller = Speller::default();
        let _spelled = speller.spell_text("Triky Custle is a funny puzzle game.").unwrap();
        let refix = Regex::new(r"(?<noun1>[a-z])(?<noun2>[A-Z])").unwrap();
        let refix1 = Regex::new(r"(?<noun1>[a-z,0-9])\.(?<noun2>[A-Z])").unwrap();
        let mut text_content = "Auto-Keras: An Eicient Neural Architecture Search System KDD ’19, August 48, 2019, Anchorage, AK, USA ACKNOWLEDGMENTS The authors thankthe anonymous reviewersfor their helpful com- ments,and allthecontributors fromtheopen-source community. Thisworkis,inpart,supportedbyDARPA(#FA8750-17-2-0116)and NSF (#IIS-1718840 and#IIS-1750074). The views, opinions, and/or ndingsexpressedarethoseoftheauthor(s)andshouldnotbe interpretedasrepresentingtheocialviewsorpoliciesoftheDe- partment of Defense or the U.S. Government. REFERENCES[1] PeterAuer, Nicolo Cesa-Bianchi,and PaulFischer.2002.Finite-timeanalysisof the multiarmed bandit problem. Machine learning (2002).[2]Bowen Baker, Otkrist Gupta, Nikhil Naik, and Ramesh Raskar. 2016.Design- ingneuralnetworkarchitecturesusingreinforcementlearning. arXivpreprint arXiv:1611.02167(2016).[3]JamesBergstra,DanYamins,andDavidDCox.2013.Hyperopt:Apythonlibrary foroptimizingthehyperparametersofmachinelearningalgorithms.In PythoninScienceConference .[4]Jean Bourgain. 1985.On Lipschitzembedding of nitemetric spacesin Hilbert space. Israel Journal of Mathematics (1985).[5]AndrewBrock,TheodoreLim,JamesMRitchie,andNickWeston.2017.SMASH: one-shotmodelarchitecturesearchthroughhypernetworks. arXivpreprint arXiv:1708.05344(2017).[6]LarsBuitinck,GillesLouppe,MathieuBlondel,FabianPedregosa,Andreas Mueller,OlivierGrisel,VladNiculae,PeterPrettenhofer,AlexandreGramfort, JaquesGrobler,et al .2013.API designformachine learningsoftware:experi- encesfromthescikit-learnproject.In ECMLPKDDWorkshop:LanguagesforData MiningandMachineLearning .[7]Han Cai, TianyaoChen, Weinan Zhang, Yong Yu,and Jun Wang. 2018.Ecient architecturesearchbynetworktransformation.In AAAIConferenceonArticial Intelligence.[8]HanCai,LigengZhu,andSongHan.2019.ProxylessNAS:Directneuralarchitec- turesearchontarget taskand hardware.In InternationalConferenceonLearning Representations .[9]Qi Chaiand GuangGong. 2012.Veriable symmetricsearchable encryptionfor semi-honest-but-curiouscloudservers. In InternationalConference onCommuni- cations.[10]Tianqi Chen, Ian Goodfellow, andJonathonShlens. 2015.Net2net: Accelerating learningviaknowledgetransfer. arXiv preprint arXiv:1511.05641 (2015).[11] Franois Chollet et al. 2015. Keras. https://keras.io. [12]TravisDesell.2017.Largescaleevolutionofconvolutionalneuralnetworks usingvolunteercomputing.In GeneticandEvolutionaryComputationConference Companion.[13]ThomasElsken,Jan-HendrikMetzen,andFrankHutter.2017.SimpleAndEf- cientArchitectureSearchfor ConvolutionalNeural Networks. arXivpreprint arXiv:1711.04528(2017).[14]ThomasElsken,JanHendrikMetzen,andFrankHutter.2018.NeuralArchitecture Search:ASurvey. arXiv preprint arXiv:1808.05377 (2018).[15]MatthiasFeurer,AaronKlein,KatharinaEggensperger,JostSpringenberg,Manuel Blum,andFrankHutter.2015.Ecientandrobustautomatedmachinelearning. InAdvances in Neural Information ProcessingSystems .[16]GolnazGhiasi,Tsung-YiLin,RuomingPang,andQuocVLe.2019.NAS-FPN: LearningScalableFeaturePyramidArchitectureforObjectDetection. arXivpreprintarXiv:1904.07392 (2019).[17]ZichaoGuo,XiangyuZhang,HaoyuanMu,WenHeng,ZechunLiu,Yichen Wei,andJianSun.2019.SinglePathOne-ShotNeuralArchitectureSearchwith UniformSampling. arXiv preprint arXiv:1904.00420 (2019).[18]BernardHaasdonkandClausBahlmann.2004.Learningwithdistancesubstitu- tionkernels.In Joint Pattern Recognition Symposium .[19]PeterEHart,NilsJNilsson,andBertramRaphael.1968. Aformalbasisforthe heuristicdetermination ofminimum costpaths. IEEEtransactions onSystems ScienceandCybernetics (1968).[20]XiaoHuang, QiangquanSong, FanYang, andXia Hu.2019.Large-scalehetero- geneousfeatureembedding.In AAAI Conference on Articial Intelligence .[21]FrankHutter,HolgerHHoos,andKevinLeyton-Brown.2011.SequentialModel- BasedOptimizationforGeneralAlgorithm Conguration.In InternationalCon- ferenceonLearningandIntelligentOptimization .[22]KirthevasanKandasamy,WillieNeiswanger,JeSchneider,BarnabasPoczos, andEricXing.2018.NeuralArchitectureSearchwithBayesianOptimisationand OptimalTransport. Advances in Neural Information ProcessingSystems (2018).[23]ScottKirkpatrick,CDanielGelatt,andMarioPVecchi.1983.Optimizationby simulatedannealing. science(1983).[24]Lars Kottho, Chris Thornton, HolgerH Hoos, Frank Hutter, and KevinLeyton- Brown.2016.Auto-WEKA2.0:Automaticmodelselectionandhyperparameter optimizationinWEKA. Journal of Machine Learning Research (2016).[25]AlexKrizhevskyandGeoreyHinton.2009. Learningmultiplelayersoffeatures fromtinyimages .TechnicalReport.Citeseer. [26]HaroldWKuhn.1955. TheHungarianmethodfortheassignmentproblem. NavalResearchLogistics (1955).[27]YannLeCun,LØonBottou,YoshuaBengio,andPatrickHaner.1998.Gradient- basedlearningappliedtodocumentrecognition. Proc. IEEE (1998).[28]ChenxiLiu,Liang-ChiehChen,FlorianSchro,HartwigAdam,WeiHua,Alan Yuille,andLiFei-Fei.2019.Auto-DeepLab:HierarchicalNeuralArchitecture SearchforSemanticImageSegmentation. arXivpreprintarXiv:1901.02985 (2019).[29]ChenxiLiu,BarretZoph,JonathonShlens,WeiHua,Li-JiaLi,LiFei-Fei,Alan Yuille,JonathanHuang,andKevinMurphy.2017.Progressiveneuralarchitecture search.In European Conference on Computer Vision .[30]Hanxiao Liu,KarenSimonyan, Oriol Vinyals,Chrisantha Fernando, andKoray Kavukcuoglu.2017.Hierarchicalrepresentationsforecientarchitecturesearch. arXivpreprintarXiv:1711.00436 (2017).[31]HanxiaoLiu,KarenSimonyan,andYimingYang.2018.Darts:Dierentiable architecturesearch. arXiv preprint arXiv:1806.09055 (2018).[32]WeiLiu,DragomirAnguelov,DumitruErhan,ChristianSzegedy,ScottReed, Cheng-YangFu,andAlexanderCBerg.2016.Ssd:Singleshotmultiboxdetector. InEuropean Conference on Computer Vision .[33]RenqianLuo,FeiTian,TaoQin,EnhongChen,andTie-YanLiu.2018.Neural architectureoptimization.In AdvancesinNeuralInformationProcessingSystems .[34]HiroshiMaehara.2013. Euclideanembeddingsofnitemetricspaces. Discrete Mathematics(2013).[35]RandalS.Olson,NathanBartley,RyanJ.Urbanowicz,andJasonH.Moore.2016. EvaluationofaTree-basedPipelineOptimizationToolforAutomatingData Science.In Genetic and Evolutionary Computation Conference 2016 .[36]FabianPedregosa,GaºlVaroquaux,AlexandreGramfort,VincentMichel, BertrandThirion, OlivierGrisel, MathieuBlondel,Peter Prettenhofer,Ron Weiss, VincentDubourg,etal .2011.Scikit-learn:MachineLearninginPython. JournalofMachineLearningResearch (2011).[37]HieuPham,MelodyYGuan,BarretZoph,QuocVLe,andJeDean.2018. EcientNeuralArchitectureSearchviaParameterSharing. arXivpreprint arXiv:1802.03268(2018).[38]Esteban Real,Alok Aggarwal,Yanping Huang,and QuocV Le.2018.Reg- ularizedEvolutionforImageClassierArchitectureSearch. arXivpreprint arXiv:1802.01548(2018).[39]EstebanReal,SherryMoore,AndrewSelle,SaurabhSaxena,YutakaLeonSue- matsu,QuocLe,andAlexKurakin.2017.Large-scaleevolutionofimage classiers,InInternationalConferenceonMachineLearning. arXivpreprint arXiv:1703.01041.[40]JasperSnoek,HugoLarochelle,andRyanPAdams.2012.Practicalbayesian optimizationofmachinelearningalgorithms.In AdvancesinNeuralInformation ProcessingSystems .[41]MasanoriSuganuma,ShinichiShirakawa,andTomoharuNagao.2017.Agenetic programmingapproachtodesigningconvolutionalneuralnetworkarchitectures. InGenetic and Evolutionary Computation Conference .[42]Mingxing Tan, Bo Chen, Ruoming Pang, Vijay Vasudevan, and Quoc V Le. 2018. Mnasnet:Platform-awareneuralarchitecturesearchformobile. arXivpreprint arXiv:1807.11626(2018).[43]QiaoyuTan,NinghaoLiu,andXiaHu.2019.DeepRepresentationLearningfor SocialNetworkAnalysis. arXiv preprint arXiv:1904.08547 (2019).[44]Chris Thornton, Frank Hutter, Holger H Hoos, and Kevin Leyton-Brown. 2013. Auto-WEKA:Combinedselectionand hyperparameteroptimizationofclassi- cationalgorithms. In InternationalConference onKnowledge Discoveryand Data Mining.[45]TaoWei,ChanghuWang,YongRui,andChangWenChen.2016.Network morphism.In International Conference on Machine Learning .[46]HanXiao,KashifRasul,andRolandVollgraf.2017.Fashion-MNIST: aNovelImageDatasetforBenchmarkingMachineLearningAlgorithms. arXiv:cs.LG/cs.LG/1708.07747[47]SiruiXie,HehuiZheng,ChunxiaoLiu,andLiangLin.2019.SNAS:stochastic neuralarchitecturesearch.In InternationalConferenceonLearningRepresenta- tions.[48]PinarYanardagandSVNVishwanathan.2015.Deepgraphkernels.In Interna-tionalConferenceonKnowledgeDiscoveryandDataMining .[49]ZhipingZeng,AnthonyKHTung,JianyongWang,JianhuaFeng,andLizhuZhou. 2009.Comparingstars:Onapproximatinggrapheditdistance.In InternationalConferenceonVeryLargeDataBases .[50]ZhaoZhong,JunjieYan,and Cheng-LinLiu. 2017.Practical Network Blocks DesignwithQ-Learning. arXiv preprint arXiv:1708.05552 (2017).[51]BarretZophandQuocVLe.2016.Neuralarchitecturesearchwithreinforcement learning.In International Conference on Learning Representations .".to_string();
        text_content = refix.replace_all(&text_content, "$non1 $noun2").to_string();
        text_content = refix1
            .replace_all(&text_content, "$non1. $noun2")
            .to_string();
        text_content = text_content.replace("- ", "");
        println!("{}", text_content);
        let _sentence: Vec<&str> = text_content.split(". ").collect();

        // for text_l in sentence {
        //     let segmented = symspell.word_segmentation(text_l, 2);
        //     println!("{}", text_l);
        //     println!("{:#?}", segmented);
        // }
    }
}
