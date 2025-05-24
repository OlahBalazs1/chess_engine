use crate::magic_bitboards::MagicHasher;
pub const ROOK_MAGIC_HASHERS: [MagicHasher; 64] = [
    MagicHasher {
        premask: 282578800148862,
        magic: 12393882513112026140,
        shift: 52,
    },
    MagicHasher {
        premask: 565157600297596,
        magic: 16300536615448215550,
        shift: 53,
    },
    MagicHasher {
        premask: 1130315200595066,
        magic: 9431097393599463424,
        shift: 53,
    },
    MagicHasher {
        premask: 2260630401190006,
        magic: 5540797841960591360,
        shift: 52,
    },
    MagicHasher {
        premask: 4521260802379886,
        magic: 7926364129043144705,
        shift: 53,
    },
    MagicHasher {
        premask: 9042521604759646,
        magic: 9691798186884073472,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043209519166,
        magic: 13790117757692214272,
        shift: 53,
    },
    MagicHasher {
        premask: 36170086419038334,
        magic: 723590819259047424,
        shift: 52,
    },
    MagicHasher {
        premask: 282578800180736,
        magic: 4164703899296597284,
        shift: 53,
    },
    MagicHasher {
        premask: 565157600328704,
        magic: 724551884886048760,
        shift: 54,
    },
    MagicHasher {
        premask: 1130315200625152,
        magic: 12863125051431666433,
        shift: 54,
    },
    MagicHasher {
        premask: 2260630401218048,
        magic: 4096586862976434172,
        shift: 54,
    },
    MagicHasher {
        premask: 4521260802403840,
        magic: 8420042517646408993,
        shift: 54,
    },
    MagicHasher {
        premask: 9042521604775424,
        magic: 8455560589070028801,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043209518592,
        magic: 17144078082013396877,
        shift: 54,
    },
    MagicHasher {
        premask: 36170086419037696,
        magic: 10390367297417448924,
        shift: 53,
    },
    MagicHasher {
        premask: 282578808340736,
        magic: 10164875046788267906,
        shift: 53,
    },
    MagicHasher {
        premask: 565157608292864,
        magic: 2482040263227261339,
        shift: 53,
    },
    MagicHasher {
        premask: 1130315208328192,
        magic: 18058557096286355842,
        shift: 54,
    },
    MagicHasher {
        premask: 2260630408398848,
        magic: 6682745912013363713,
        shift: 54,
    },
    MagicHasher {
        premask: 4521260808540160,
        magic: 13468826125535293953,
        shift: 54,
    },
    MagicHasher {
        premask: 9042521608822784,
        magic: 10611055160617702032,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043209388032,
        magic: 17767584537420769290,
        shift: 54,
    },
    MagicHasher {
        premask: 36170086418907136,
        magic: 18220400809065320277,
        shift: 53,
    },
    MagicHasher {
        premask: 282580897300736,
        magic: 6687967543972211639,
        shift: 53,
    },
    MagicHasher {
        premask: 565159647117824,
        magic: 2344551376201318465,
        shift: 54,
    },
    MagicHasher {
        premask: 1130317180306432,
        magic: 13422632479365242956,
        shift: 53,
    },
    MagicHasher {
        premask: 2260632246683648,
        magic: 14741496629853405285,
        shift: 53,
    },
    MagicHasher {
        premask: 4521262379438080,
        magic: 18291377108039020626,
        shift: 53,
    },
    MagicHasher {
        premask: 9042522644946944,
        magic: 4402815333564416865,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043175964672,
        magic: 4614907845197434983,
        shift: 54,
    },
    MagicHasher {
        premask: 36170086385483776,
        magic: 13450837298958666765,
        shift: 53,
    },
    MagicHasher {
        premask: 283115671060736,
        magic: 7317688161780353748,
        shift: 53,
    },
    MagicHasher {
        premask: 565681586307584,
        magic: 17736436826450264254,
        shift: 53,
    },
    MagicHasher {
        premask: 1130822006735872,
        magic: 15875611869642248256,
        shift: 54,
    },
    MagicHasher {
        premask: 2261102847592448,
        magic: 6907071372919752043,
        shift: 53,
    },
    MagicHasher {
        premask: 4521664529305600,
        magic: 8529128498564953876,
        shift: 53,
    },
    MagicHasher {
        premask: 9042787892731904,
        magic: 5182521943789668955,
        shift: 53,
    },
    MagicHasher {
        premask: 18085034619584512,
        magic: 17445038258271782881,
        shift: 53,
    },
    MagicHasher {
        premask: 36170077829103616,
        magic: 1162558786460705081,
        shift: 52,
    },
    MagicHasher {
        premask: 420017753620736,
        magic: 5334455153081559900,
        shift: 52,
    },
    MagicHasher {
        premask: 699298018886144,
        magic: 10944169556394554419,
        shift: 53,
    },
    MagicHasher {
        premask: 1260057572672512,
        magic: 9690766393682492679,
        shift: 53,
    },
    MagicHasher {
        premask: 2381576680245248,
        magic: 5670019441515688490,
        shift: 53,
    },
    MagicHasher {
        premask: 4624614895390720,
        magic: 13764970747231374109,
        shift: 53,
    },
    MagicHasher {
        premask: 9110691325681664,
        magic: 8766725104060705281,
        shift: 53,
    },
    MagicHasher {
        premask: 18082844186263552,
        magic: 9757764282873560930,
        shift: 53,
    },
    MagicHasher {
        premask: 36167887395782656,
        magic: 16710162559125369229,
        shift: 52,
    },
    MagicHasher {
        premask: 35466950888980736,
        magic: 7791134854616239104,
        shift: 53,
    },
    MagicHasher {
        premask: 34905104758997504,
        magic: 7926329556047165904,
        shift: 54,
    },
    MagicHasher {
        premask: 34344362452452352,
        magic: 13473364088356958976,
        shift: 54,
    },
    MagicHasher {
        premask: 33222877839362048,
        magic: 6557239468353762816,
        shift: 54,
    },
    MagicHasher {
        premask: 30979908613181440,
        magic: 12033618006761920000,
        shift: 54,
    },
    MagicHasher {
        premask: 26493970160820224,
        magic: 12114120288466900480,
        shift: 54,
    },
    MagicHasher {
        premask: 17522093256097792,
        magic: 7083168736482374656,
        shift: 54,
    },
    MagicHasher {
        premask: 35607136465616896,
        magic: 1020723420621721088,
        shift: 53,
    },
    MagicHasher {
        premask: 9079539427579068672,
        magic: 520938430754667148,
        shift: 52,
    },
    MagicHasher {
        premask: 8935706818303361536,
        magic: 5872691426220854398,
        shift: 53,
    },
    MagicHasher {
        premask: 8792156787827803136,
        magic: 12213762225165068490,
        shift: 53,
    },
    MagicHasher {
        premask: 8505056726876686336,
        magic: 16429131068009480110,
        shift: 53,
    },
    MagicHasher {
        premask: 7930856604974452736,
        magic: 7174797260443623482,
        shift: 53,
    },
    MagicHasher {
        premask: 6782456361169985536,
        magic: 13781014878279116802,
        shift: 53,
    },
    MagicHasher {
        premask: 4485655873561051136,
        magic: 3223451510870575148,
        shift: 54,
    },
    MagicHasher {
        premask: 9115426935197958144,
        magic: 16570994835554228494,
        shift: 53,
    },
];

pub const BISHOP_MAGIC_HASHERS: [MagicHasher; 64] = [
    MagicHasher {
        premask: 18049651735527936,
        magic: 6791474864611757060,
        shift: 59,
    },
    MagicHasher {
        premask: 70506452091904,
        magic: 7499112530489114448,
        shift: 60,
    },
    MagicHasher {
        premask: 275415828992,
        magic: 8622271177600044493,
        shift: 59,
    },
    MagicHasher {
        premask: 1075975168,
        magic: 4469827659624412402,
        shift: 59,
    },
    MagicHasher {
        premask: 38021120,
        magic: 2813067585713127408,
        shift: 59,
    },
    MagicHasher {
        premask: 8657588224,
        magic: 2271653026214686767,
        shift: 59,
    },
    MagicHasher {
        premask: 2216338399232,
        magic: 1149542245494416309,
        shift: 60,
    },
    MagicHasher {
        premask: 567382630219776,
        magic: 16628976621147062280,
        shift: 59,
    },
    MagicHasher {
        premask: 9024825867763712,
        magic: 3036845381962635256,
        shift: 60,
    },
    MagicHasher {
        premask: 18049651735527424,
        magic: 10990688979381298174,
        shift: 60,
    },
    MagicHasher {
        premask: 70506452221952,
        magic: 7723348419871505200,
        shift: 59,
    },
    MagicHasher {
        premask: 275449643008,
        magic: 6241786816352721112,
        shift: 59,
    },
    MagicHasher {
        premask: 9733406720,
        magic: 17419384675554449910,
        shift: 59,
    },
    MagicHasher {
        premask: 2216342585344,
        magic: 5337629797065435516,
        shift: 59,
    },
    MagicHasher {
        premask: 567382630203392,
        magic: 7528990312611872434,
        shift: 60,
    },
    MagicHasher {
        premask: 1134765260406784,
        magic: 4831448365257736157,
        shift: 60,
    },
    MagicHasher {
        premask: 4512412933816832,
        magic: 4449600180738183149,
        shift: 60,
    },
    MagicHasher {
        premask: 9024825867633664,
        magic: 9340486265188227049,
        shift: 60,
    },
    MagicHasher {
        premask: 18049651768822272,
        magic: 887211635167785186,
        shift: 57,
    },
    MagicHasher {
        premask: 70515108615168,
        magic: 9544256224793467780,
        shift: 57,
    },
    MagicHasher {
        premask: 2491752130560,
        magic: 3147454065829610951,
        shift: 57,
    },
    MagicHasher {
        premask: 567383701868544,
        magic: 4529073186046386177,
        shift: 57,
    },
    MagicHasher {
        premask: 1134765256220672,
        magic: 16308664803584802627,
        shift: 60,
    },
    MagicHasher {
        premask: 2269530512441344,
        magic: 7154531282928214007,
        shift: 60,
    },
    MagicHasher {
        premask: 2256206450263040,
        magic: 7941020453771886550,
        shift: 59,
    },
    MagicHasher {
        premask: 4512412900526080,
        magic: 17982833550070901608,
        shift: 59,
    },
    MagicHasher {
        premask: 9024834391117824,
        magic: 3226837982187333601,
        shift: 57,
    },
    MagicHasher {
        premask: 18051867805491712,
        magic: 8997191253756682568,
        shift: 54,
    },
    MagicHasher {
        premask: 637888545440768,
        magic: 18081824963186155530,
        shift: 55,
    },
    MagicHasher {
        premask: 1135039602493440,
        magic: 7875817186126873096,
        shift: 57,
    },
    MagicHasher {
        premask: 2269529440784384,
        magic: 9482350501171344385,
        shift: 59,
    },
    MagicHasher {
        premask: 4539058881568768,
        magic: 16239430089774489560,
        shift: 59,
    },
    MagicHasher {
        premask: 1128098963916800,
        magic: 9324769480510895688,
        shift: 59,
    },
    MagicHasher {
        premask: 2256197927833600,
        magic: 4539923232720836613,
        shift: 59,
    },
    MagicHasher {
        premask: 4514594912477184,
        magic: 844416220665086977,
        shift: 57,
    },
    MagicHasher {
        premask: 9592139778506752,
        magic: 14928089520601402467,
        shift: 54,
    },
    MagicHasher {
        premask: 19184279556981248,
        magic: 4356111086088622146,
        shift: 54,
    },
    MagicHasher {
        premask: 2339762086609920,
        magic: 12377571792544000818,
        shift: 56,
    },
    MagicHasher {
        premask: 4538784537380864,
        magic: 13768186248078761218,
        shift: 58,
    },
    MagicHasher {
        premask: 9077569074761728,
        magic: 2896716498385559933,
        shift: 58,
    },
    MagicHasher {
        premask: 562958610993152,
        magic: 7318317858568518858,
        shift: 59,
    },
    MagicHasher {
        premask: 1125917221986304,
        magic: 13338101209057593194,
        shift: 59,
    },
    MagicHasher {
        premask: 2814792987328512,
        magic: 451117367685367504,
        shift: 56,
    },
    MagicHasher {
        premask: 5629586008178688,
        magic: 4399738569223932336,
        shift: 56,
    },
    MagicHasher {
        premask: 11259172008099840,
        magic: 12815110503711155628,
        shift: 56,
    },
    MagicHasher {
        premask: 22518341868716544,
        magic: 12737021131892731929,
        shift: 56,
    },
    MagicHasher {
        premask: 9007336962655232,
        magic: 9223257213191261196,
        shift: 59,
    },
    MagicHasher {
        premask: 18014673925310464,
        magic: 17239664914785975212,
        shift: 59,
    },
    MagicHasher {
        premask: 2216338399232,
        magic: 1105632240994323946,
        shift: 59,
    },
    MagicHasher {
        premask: 4432676798464,
        magic: 15617356390490049760,
        shift: 59,
    },
    MagicHasher {
        premask: 11064376819712,
        magic: 14471330052756067262,
        shift: 58,
    },
    MagicHasher {
        premask: 22137335185408,
        magic: 9774271290522035875,
        shift: 58,
    },
    MagicHasher {
        premask: 44272556441600,
        magic: 18182382925105383297,
        shift: 58,
    },
    MagicHasher {
        premask: 87995357200384,
        magic: 16768221405807302376,
        shift: 58,
    },
    MagicHasher {
        premask: 35253226045952,
        magic: 10030303534279217359,
        shift: 59,
    },
    MagicHasher {
        premask: 70506452091904,
        magic: 7782132755381280109,
        shift: 59,
    },
    MagicHasher {
        premask: 567382630219776,
        magic: 1371911262438699025,
        shift: 58,
    },
    MagicHasher {
        premask: 1134765260406784,
        magic: 9065970347511490345,
        shift: 59,
    },
    MagicHasher {
        premask: 2832480465846272,
        magic: 5675639115175713521,
        shift: 58,
    },
    MagicHasher {
        premask: 5667157807464448,
        magic: 3896404074769458234,
        shift: 58,
    },
    MagicHasher {
        premask: 11333774449049600,
        magic: 5936855253689467265,
        shift: 59,
    },
    MagicHasher {
        premask: 22526811443298304,
        magic: 6405295973796135651,
        shift: 59,
    },
    MagicHasher {
        premask: 9024825867763712,
        magic: 9088404355728160208,
        shift: 60,
    },
    MagicHasher {
        premask: 18049651735527936,
        magic: 4828193461272579640,
        shift: 59,
    },
];
