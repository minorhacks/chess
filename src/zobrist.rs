use crate::castle_rights::CastleRights;
use crate::color::{Color, NUM_COLORS};
use crate::file::{File, NUM_FILES};
use crate::piece::{Piece, NUM_PIECES};
use crate::square::{Square, NUM_SQUARES};

/// Create a completely blank type.  This allows all the functions to be part of this type, which I
/// think is a bit cleaner than bare functions everywhere.
pub struct Zobrist;

// Include the generated lookup tables
const SIDE_TO_MOVE: u64 = 4527170993230009529;

const ZOBRIST_PIECES: [[[u64; NUM_SQUARES]; NUM_PIECES]; NUM_COLORS] = [[[
    8085185598151760447,
    13946325585335384165,
    2968751616440035579,
    7041022842525770343,
    12917422304011698271,
    10052360050776251686,
    13699342445398323639,
    506573857888973691,
    11886874439810951006,
    13321128211373863361,
    8759922935711139237,
    6272023614444296008,
    10763222970733438174,
    16100965241052210334,
    7335507063161343948,
    4437339628074423870,
    8394786845659756955,
    17278309931297955272,
    15344764256493080466,
    5060076868008773000,
    12887984606205686529,
    10378348301582331723,
    16654624460559881141,
    5562172382807043852,
    5092287325892671854,
    13396853578277207988,
    14844752070250998485,
    1373098290062978218,
    6329027450582441694,
    7087920059585399402,
    10551797782285515443,
    9615121390358484351,
    709948133425849678,
    6423043237130300195,
    3008006698062651143,
    233213343324799528,
    7690788824206336646,
    18417575893370114286,
    4512104827508390788,
    13109509118233219187,
    14903015139044807688,
    3522116723393850159,
    11182691149835678343,
    14407773952884059449,
    2181417584272620251,
    17961847337695662156,
    14098576545695010209,
    15671501844129536668,
    10510435711806366109,
    5675830999581763190,
    7376863761877281391,
    17635353422420823681,
    14720315539620897069,
    7458002836843723420,
    2368817349156128604,
    18060931465287773585,
    15563011342426703550,
    7775944634769904345,
    3426351430552521287,
    10311865472744934538,
    11781042725034006691,
    4905555050952954108,
    4333776149487132986,
    2463367108216104083,
   ], [
    13599402234106691047,
    13450911146927257322,
    3511137665223944314,
    17729539245871410786,
    8169896905271198830,
    3746689924575831060,
    8072384364126190005,
    4011870641253644118,
    16838769081454081609,
    18378586487414723725,
    11221550773678523470,
    10119731459805208541,
    3344814324184951726,
    15086803744157759816,
    17840335201726031357,
    15573097248123143676,
    7307230178222424946,
    1611977266260903713,
    15490536914568105123,
    7516674779991348384,
    13665394482510419633,
    6089563121758516729,
    14711127434108685208,
    13705599115060744804,
    6125962466729398413,
    1349302667717379749,
    14275899594006570154,
    6471541157280608695,
    6707496620449959713,
    17361391307182139287,
    5737035911449072716,
    3336945886184819959,
    1636376456350229463,
    3595699902859461573,
    5090639554733180836,
    1777271225210662942,
    12503984654916127053,
    3993496153543331097,
    9093000627561052665,
    3959630880763955094,
    6323846231988685160,
    5881728293156563380,
    7195523231502221581,
    13990380562917067025,
    2907646858231148552,
    11171916760475011863,
    3547283148093525189,
    4012919210417812347,
    11492905190446805694,
    9307971329815009137,
    14262833555050728970,
    10377008545509186679,
    5635257166441904749,
    14534145938775712324,
    5614004858224860348,
    858423430941191831,
    12293725579838946514,
    10825275950904701112,
    14557365833283554744,
    7415850845694747135,
    11253919690577557185,
    8680038447064542190,
    4013364551032066478,
    12807305106885750696,
   ], [
    5791572184776859174,
    12024733143534293352,
    7864225555979988004,
    3736401823337381981,
    7574690434978985387,
    6687964274233157365,
    6957055408118596253,
    2790098368799341557,
    6332475251422498985,
    15591415144634489646,
    4211391684596661941,
    4972987153826539359,
    11613947630260637501,
    13044510836649692787,
    17936783429747107791,
    9289801971334000425,
    9557733891569947375,
    12630328621565084034,
    12264112490790636651,
    11239788002682247786,
    13645662263780506469,
    10809503168209127681,
    13743367264077317594,
    12831656190331824099,
    12276231546426300063,
    3865813547281979457,
    2254162467496427405,
    17470816890398757602,
    16516641544014017489,
    5570645727585239858,
    13539449535017755504,
    6804328412441181293,
    16919278166962233256,
    12190724817127772349,
    5882312003247831308,
    14525233436761851379,
    10367425884471880865,
    7602604110826236233,
    7376940435100392887,
    15513861237954440936,
    18425651988518136890,
    11557105086048811701,
    13356861099170317282,
    3216366742695841914,
    3797137306265862784,
    13521902207328083180,
    10504631681750921332,
    12484565884266753853,
    1502294742856228971,
    16420037132225047539,
    8147387324392529164,
    12798367230622346536,
    16773259956348296769,
    2369507822693470161,
    11858845087814641986,
    2019456586782465834,
    8791297079776423265,
    15850745899410092106,
    5810866027600402117,
    271465086763610880,
    8493946749610034811,
    14941278182291072990,
    11184476609784827847,
    2913374001864755334,
   ], [
    10649285276686460395,
    2466225003344230383,
    14671767597778217728,
    14919605551303920302,
    16761774854545928513,
    9330370944484848459,
    5608302011273423023,
    17813982169043092628,
    11914502131398296965,
    13583526454055017493,
    5882346319633422312,
    77368525960830867,
    11480747378357980303,
    9346868888074220080,
    18128862089844542808,
    14666200809904390097,
    5162427499596212771,
    10255935744797817480,
    6110979322803042822,
    2831041740759397137,
    10956236347537763342,
    13933701026148623147,
    3983839483112559136,
    18090477929563539577,
    6096722601825392602,
    16579206072354233411,
    14933535764166314850,
    10708409047466984175,
    16544662100990687221,
    2143038002464035034,
    2122831342527274681,
    15455102092924629400,
    17308554294472212849,
    9630058838831792128,
    9932553949255390014,
    11674564156962281505,
    4937261759390827155,
    6938691795422267736,
    17536753754354241661,
    2880254870281436113,
    16996830693264743308,
    2788447340894240099,
    2308087642812475358,
    12353095302557963504,
    16518342919522783782,
    15608680744866559613,
    12735302370504556730,
    6826859169588633165,
    765112237812162241,
    2562788983976407713,
    12976527389869081433,
    14052505272918952173,
    16639441684377152380,
    6700627022284040941,
    7480559834028999034,
    10948909398747033120,
    5741614840165464187,
    13471691637077969891,
    17036652965546292439,
    5690124785098181473,
    9163274257046637988,
    9110941231833595613,
    1353097680410037206,
    4483566134978307125,
   ], [
    15191445493152138885,
    9681555156338762403,
    15366694896250378880,
    15503611236075558766,
    15137205987365457276,
    14884077914095346923,
    8985290104445813869,
    5777992265227864191,
    14284985605626534542,
    12812785393908790533,
    9579503051817138331,
    5392854559903396351,
    6723704436626733384,
    13331195011056686554,
    5775775053761472559,
    5943212329767160200,
    1781546323060871614,
    3672572168488118376,
    13019784582250485763,
    13932860670837009106,
    17247882515431000907,
    5834799761991789951,
    15278159737734340940,
    1089062936140719283,
    13696370045006852500,
    70265523359545266,
    35451223986256226,
    1492939210641070727,
    11884044008494162661,
    15377331101188118145,
    8424274752374325492,
    3309496476538694153,
    8405548710512920964,
    3694468651343360489,
    14926604008424952173,
    16216242213689336640,
    1971819447144777776,
    17598022412216810782,
    9250673106350260646,
    7049160027093673291,
    4084144104454406822,
    12019090282815043147,
    1278634565271369615,
    14532740883218741004,
    7394625974387494398,
    7478432795313581916,
    1344503391170569378,
    2590648718103572115,
    10831199651266356381,
    17420437432191615856,
    15775288814409278988,
    17475925489104274686,
    16702253895089406273,
    14236444189252778008,
    6122133546869650768,
    12236085381393819465,
    6141798004589552157,
    12068395932684116817,
    8557807252518345166,
    3029206766081152815,
    15341371467755788230,
    15037171718890640016,
    12550560721888003138,
    9774973148039007930,
   ], [
    7491554540666671112,
    14459112677601979903,
    15879518618381949993,
    14180864030909484092,
    9946669269017291608,
    11279591070152456800,
    15201552162696831992,
    4495460355923658287,
    7984168475443507802,
    18120778839617424897,
    8265802707549396213,
    957322794002697711,
    2645235880175942022,
    2580813431052431212,
    17056084610116992261,
    15354103500888498974,
    2943044645951743254,
    7978564978448347271,
    2038225337494207362,
    6556844033253841485,
    2424746509898573603,
    1442471486894969034,
    9428016994712628171,
    3393584574301548502,
    4331398012230501043,
    10747892437744473901,
    62422429814636341,
    3938116414905347908,
    2829023869595979569,
    1956619858501177213,
    10248706421585488791,
    86630683577193141,
    13307716120118605834,
    12864872728076558002,
    14758240653735500638,
    2788257879909482104,
    16415847132231327533,
    14757462880665409217,
    3673654887415232891,
    1499957626664400686,
    1807581277916024363,
    690898649642943294,
    4593876493217043157,
    9639397867802271120,
    1375658789399403022,
    7245808033510309305,
    8869259659731437534,
    2156841209831332319,
    574553234955511055,
    137430486685106463,
    1904833687846081636,
    18154198398899680172,
    7894885270199010912,
    5227601755901285816,
    7270473610383991417,
    4335458999701163313,
    3908016979908587907,
    8522998408369993737,
    2854606090194844392,
    14150441652227720703,
    7563608754644160471,
    6534089586418192894,
    15140048132436079644,
    2583829521678241116,
  ]], [[
    1974119971942181497,
    12122172008208551821,
    6062452120433435846,
    18396934563637245408,
    8044283600526533767,
    11399642828105987255,
    8987578966063644515,
    11056374512524529789,
    2095926713241145349,
    15632575041811929556,
    14766238854338441338,
    17890133514240121960,
    4376355677015825915,
    890288126683688722,
    6939599344405662403,
    18244768664859276680,
    3536795021499152893,
    12730147489584249157,
    6591604030498980534,
    3332675066383947135,
    6699201019113945888,
    9282854365328689576,
    9271666704537739704,
    6713588716128219818,
    12875847909595158622,
    2844291356417016583,
    13761885081363751180,
    17215576610366625388,
    14594744591378946410,
    2712865253957514908,
    15602993547219531013,
    5876508857414472499,
    2263777162867217488,
    14064797817611449044,
    1639959676191337423,
    12462746736495809728,
    17679389401087310828,
    5728471850377755580,
    834894166465115306,
    5025494886860419275,
    13888363611479590573,
    2683611855720127261,
    3382535446547267230,
    10049447372213223528,
    6992881842228855765,
    6718851911638198654,
    7944752306398536041,
    6557650052889617593,
    11550969363163114495,
    3607248707049579728,
    15951684984257691760,
    3342710414128087565,
    16532053739358337567,
    13350120864672994269,
    12060557997550275081,
    1940730362432768231,
    4892474443799080794,
    18363889940509909703,
    14286311574710607121,
    898492672058578451,
    9766764379781634153,
    6260715823505312240,
    539984552968257160,
    16594171728389449529,
   ], [
    10455513130809424773,
    17623806341049643038,
    3659277771740041156,
    8246986533495291870,
    1648081958666603438,
    3036643700894255662,
    7112138556000586248,
    17816539284358262381,
    5013945729082421901,
    14285782936934827911,
    11291941467650496583,
    10112661610633844762,
    15627711145565000939,
    1043450519575561958,
    15337132795904165921,
    3306146212917133088,
    7066207104219760399,
    14308741701940577641,
    9540423879177815217,
    12655019185473569072,
    231757402935533483,
    15252568062546531527,
    15844705444959748171,
    4107643414615621211,
    4209324916786452211,
    10080270213726738271,
    15507236651564249381,
    2830118014489052027,
    1272429644337854603,
    5863312997442792627,
    6669408100564662870,
    17852622614246750198,
    426057782112405669,
    9169275361207959466,
    16474682595577495914,
    651265388359517841,
    2652160390022101965,
    596299090000166449,
    5390060501359783226,
    9885856619764708136,
    15633504720926951697,
    4107468267441282007,
    18171669310023179356,
    10587946152207716915,
    8130850740471361717,
    5291377364390306530,
    10400583777616926355,
    15574501996960938443,
    14718787363374214390,
    2316346396027588423,
    11441439282626508057,
    9685852710691429773,
    13536448040647565144,
    3281847316116869295,
    17130353528622791980,
    14944124375728012798,
    16249643864573951410,
    14642164706219003814,
    14146227915698410317,
    834923297581854918,
    2243945603276884596,
    18294374039000095626,
    14342779990852108980,
    12349388067280678975,
   ], [
    4788847924633284803,
    8775275993708565302,
    4130262079753223056,
    10348118816783317010,
    410918958768738239,
    7001106083584572275,
    14958613116238175136,
    1279694312272707306,
    11218380613939408847,
    181344061445751182,
    11127306649089829538,
    13156365405071069825,
    2928652815813366643,
    7080125573750053812,
    18045886575860516642,
    774142121406799373,
    10775758583235566634,
    1735086817014723169,
    234611643265364591,
    12234227122460528908,
    10341599500877707634,
    8000178073288307561,
    4907455879075177993,
    13038497872840189399,
    111034178850992603,
    16390072115841390503,
    2776841493158126518,
    11925304704145098863,
    16099639151712531547,
    357459391900979914,
    9491000353623016707,
    4789843412420081769,
    3765898457177677613,
    7843265043142146945,
    12322134860489944219,
    5594374085690979333,
    4662912630045528703,
    18222868451136218862,
    5328483134285744109,
    13230633928370631422,
    4550736427673407556,
    5516106228715860714,
    10133695524959120482,
    15613570228627561752,
    6541044483842538045,
    15367202567798209048,
    9603650400754005271,
    18344949126797252199,
    10515720614944474142,
    15712905308579626888,
    2519673912863200504,
    11776877319727849355,
    10490586897351449945,
    12912594103356369237,
    2024947851179874453,
    15789914074529688704,
    11278001377160540923,
    5371118178124159195,
    14707504626623780328,
    9690131794191051626,
    14850615933097402282,
    6834136278343025907,
    15884829212272191591,
    12993172404731853696,
   ], [
    3895792506770256634,
    9245323961033928241,
    1313790020052063315,
    11527279242489925295,
    4863732858002746390,
    12407749236081454279,
    2253802418203609837,
    1400751866855241176,
    1893492991331732339,
    14916816832865118066,
    13843141957951860147,
    4041669728575725925,
    16621799766708814189,
    2280953044639285490,
    15222790734766566032,
    13904793611633027402,
    4590452674074552886,
    5971062380966608376,
    4225194491320456852,
    947680832864465008,
    13705257119599998067,
    17276097317142303983,
    13813883793458298328,
    9448258496511070816,
    11892813532712038540,
    15417074628318613278,
    17266501981826408257,
    6524544751792064405,
    11657477378573416444,
    16357886020300469389,
    16869150268900580345,
    1854437118731610840,
    17267470854844366310,
    6348806534885290172,
    15987267695269561595,
    12052963883910886186,
    9606918784172406842,
    17813123067495634904,
    2083245589483807330,
    8412452879489357271,
    1663615316053847826,
    16690700182929364056,
    15632881909859024971,
    2199950710187759964,
    1207181948565839773,
    7591168420664542281,
    8682806408846780720,
    4765403109304955260,
    16756072415699188519,
    2226244971568525971,
    14451915854280980173,
    8578598917165754216,
    4936970355708472178,
    14664596822180234239,
    8368330620392729098,
    17097109877852273603,
    4347880792516783399,
    9679292567523676236,
    6613155920564355164,
    6703955889874475957,
    1176247665222520005,
    14398812172304171969,
    11905489411344525085,
    18158351593698301620,
   ], [
    10034950551158140841,
    1496691604118684084,
    18143055571889881107,
    17770693203069037586,
    17317431568687125209,
    18278813171271916533,
    2359242116427935738,
    785542510366413819,
    17301405340058624745,
    7140269173598887326,
    15763147094133758797,
    17019590163056549422,
    8313187747808243070,
    9671637864551433554,
    5903124771413257696,
    82602691264698670,
    14022313277735186726,
    17140379202737090177,
    16293971375789431514,
    5639005867879826787,
    15263066725688832824,
    7206484280905107426,
    629907202506466403,
    4422970517779353802,
    7522835645578898075,
    16440255583399783099,
    14327981882115497883,
    3374301353053585308,
    14873693955201124886,
    17674116492679490938,
    5413655290396399269,
    4377225530767239308,
    606731548081487279,
    11501342163862926732,
    5481882681212053468,
    5890258737783298415,
    7608464852375703539,
    16200161143838363103,
    6128116976673623993,
    12316970932758876305,
    18258991022559553905,
    7376450170906431622,
    3730387996326798301,
    9597624152492977110,
    8962842035019961412,
    15415970453573393038,
    10832339036587530375,
    247108704137585621,
    15294605457940569598,
    6775317081669210359,
    3357383835561110217,
    3607670381479862047,
    3110458140293103368,
    4013724623671866380,
    1102386300661769945,
    6874360661426331622,
    5407731031126341176,
    5933752742580668438,
    3855210440432193104,
    17501959345119395936,
    7851063102274343031,
    5507359404973964554,
    7288647810171171732,
    9963264888239115480,
   ], [
    17169988300180501737,
    726447829084060427,
    10340748299739480700,
    7283847359867718292,
    10479651240042143347,
    2441440262675529156,
    18393658032926132501,
    4638369152871654049,
    10513741389883296149,
    16461198869304007361,
    4202861081018730262,
    14913492818387112274,
    1362766754278226290,
    11167858548289520024,
    12938351619060419059,
    15923654670843077062,
    11220100058155943933,
    1118424369306173434,
    6913652011578739811,
    2986354088008024484,
    5291848334770914160,
    5002824289243929537,
    5449873472667164622,
    16783466385053769937,
    12206096687024787939,
    11338032047932101346,
    3281278097324636579,
    7970008097319910597,
    15492612253978767971,
    4911796366367885599,
    14720744053492590884,
    1499684767450747270,
    2753158744217262796,
    10701917121957719730,
    290440056106097495,
    15119800239453701342,
    10656676603090838398,
    4655908181986731505,
    9875671628831054291,
    6863327516456157024,
    10262619563867612375,
    2023991474017457276,
    11910561862303641638,
    15066815237549999891,
    16344770723995001336,
    1198516712141541470,
    13578283652565471980,
    8217304346012117096,
    18069605163778075845,
    11078815593591841764,
    9861709967479056517,
    11178355992962319876,
    9225125148467968870,
    2083458366328509072,
    3789655262776435400,
    15261100874054714676,
    14834839189171198411,
    10108133918651475670,
    17766420056753101734,
    15741601523989260314,
    14499901814074686751,
    12103964567632648229,
    8029469496028142804,
    17154414655041112320,
]]];

const ZOBRIST_CASTLES: [[u64; 4]; NUM_COLORS] = [[
    4730299224526584862,
    16029894244043714123,
    12083657999841033022,
    18076508660253391372,
  ], [
    2415293488023962265,
    16644142240967013679,
    4034016745971452800,
    9785248589528863946,
]];

const ZOBRIST_EP: [[u64; NUM_FILES]; NUM_COLORS] = [[
    9435306652719281316,
    7508281906614734953,
    10242866157889915384,
    3430736765343475721,
    14246344884062566213,
    7039000897023772337,
    15998493998808429983,
    3681690809747865492,
], [
    14723149858738564849,
    14795388801617171918,
    3138649028811902330,
    8925713794849363562,
    16544578013666791189,
    2352122330455272730,
    14186660030116373031,
    16795260471835440987,
]];



impl Zobrist {
    /// Get the value for a particular piece
    #[inline]
    pub fn piece(piece: Piece, square: Square, color: Color) -> u64 {
        unsafe {
            *ZOBRIST_PIECES
                .get_unchecked(color.to_index())
                .get_unchecked(piece.to_index())
                .get_unchecked(square.to_index())
        }
    }

    #[inline]
    pub fn castles(castle_rights: CastleRights, color: Color) -> u64 {
        unsafe {
            *ZOBRIST_CASTLES
                .get_unchecked(color.to_index())
                .get_unchecked(castle_rights.to_index())
        }
    }

    #[inline]
    pub fn en_passant(file: File, color: Color) -> u64 {
        unsafe {
            *ZOBRIST_EP
                .get_unchecked(color.to_index())
                .get_unchecked(file.to_index())
        }
    }

    #[inline]
    pub fn color() -> u64 {
        SIDE_TO_MOVE
    }
}
