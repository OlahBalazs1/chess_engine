use crate::magic_bitboards::MagicHasher;

pub const ROOK_MAGIC_HASHERS: [MagicHasher; 64] = [
    MagicHasher {
        premask: 282578800148862,
        magic: 4970501634360698732,
        shift: 48,
    },
    MagicHasher {
        premask: 565157600297596,
        magic: 11612673227973068494,
        shift: 49,
    },
    MagicHasher {
        premask: 1130315200595066,
        magic: 2130901070441978214,
        shift: 49,
    },
    MagicHasher {
        premask: 2260630401190006,
        magic: 14113223944283661404,
        shift: 49,
    },
    MagicHasher {
        premask: 4521260802379886,
        magic: 12142565806889963941,
        shift: 50,
    },
    MagicHasher {
        premask: 9042521604759646,
        magic: 1902525151263403624,
        shift: 49,
    },
    MagicHasher {
        premask: 18085043209519166,
        magic: 16848667412961013506,
        shift: 50,
    },
    MagicHasher {
        premask: 36170086419038334,
        magic: 15824475794810749338,
        shift: 48,
    },
    MagicHasher {
        premask: 282578800180736,
        magic: 12199756656616948173,
        shift: 49,
    },
    MagicHasher {
        premask: 565157600328704,
        magic: 17725863309640774639,
        shift: 50,
    },
    MagicHasher {
        premask: 1130315200625152,
        magic: 15256933246226838370,
        shift: 51,
    },
    MagicHasher {
        premask: 2260630401218048,
        magic: 4347142485805088950,
        shift: 50,
    },
    MagicHasher {
        premask: 4521260802403840,
        magic: 13387710321532760081,
        shift: 50,
    },
    MagicHasher {
        premask: 9042521604775424,
        magic: 4523403256833128792,
        shift: 50,
    },
    MagicHasher {
        premask: 18085043209518592,
        magic: 5960041289466910871,
        shift: 51,
    },
    MagicHasher {
        premask: 36170086419037696,
        magic: 205888692177550452,
        shift: 49,
    },
    MagicHasher {
        premask: 282578808340736,
        magic: 8036688626283983578,
        shift: 50,
    },
    MagicHasher {
        premask: 565157608292864,
        magic: 11097774322382453952,
        shift: 51,
    },
    MagicHasher {
        premask: 1130315208328192,
        magic: 18395727697925216936,
        shift: 50,
    },
    MagicHasher {
        premask: 2260630408398848,
        magic: 3547351534034877188,
        shift: 50,
    },
    MagicHasher {
        premask: 4521260808540160,
        magic: 8485767490770710768,
        shift: 50,
    },
    MagicHasher {
        premask: 9042521608822784,
        magic: 1845378037744015874,
        shift: 50,
    },
    MagicHasher {
        premask: 18085043209388032,
        magic: 16536673877024159257,
        shift: 50,
    },
    MagicHasher {
        premask: 36170086418907136,
        magic: 10848230501007100647,
        shift: 50,
    },
    MagicHasher {
        premask: 282580897300736,
        magic: 5528910855709001195,
        shift: 49,
    },
    MagicHasher {
        premask: 565159647117824,
        magic: 10820457266849247129,
        shift: 51,
    },
    MagicHasher {
        premask: 1130317180306432,
        magic: 2199422656438680455,
        shift: 50,
    },
    MagicHasher {
        premask: 2260632246683648,
        magic: 8476071152637714059,
        shift: 50,
    },
    MagicHasher {
        premask: 4521262379438080,
        magic: 18048436469202169805,
        shift: 50,
    },
    MagicHasher {
        premask: 9042522644946944,
        magic: 13730182701046332919,
        shift: 50,
    },
    MagicHasher {
        premask: 18085043175964672,
        magic: 7571103917092232853,
        shift: 50,
    },
    MagicHasher {
        premask: 36170086385483776,
        magic: 15924776538930456857,
        shift: 49,
    },
    MagicHasher {
        premask: 283115671060736,
        magic: 12863636726085757424,
        shift: 49,
    },
    MagicHasher {
        premask: 565681586307584,
        magic: 1477197738909274028,
        shift: 51,
    },
    MagicHasher {
        premask: 1130822006735872,
        magic: 6427356768060049143,
        shift: 50,
    },
    MagicHasher {
        premask: 2261102847592448,
        magic: 1350491632821510604,
        shift: 50,
    },
    MagicHasher {
        premask: 4521664529305600,
        magic: 17932165805436809532,
        shift: 50,
    },
    MagicHasher {
        premask: 9042787892731904,
        magic: 1804098588416070376,
        shift: 50,
    },
    MagicHasher {
        premask: 18085034619584512,
        magic: 3271827724112274682,
        shift: 52,
    },
    MagicHasher {
        premask: 36170077829103616,
        magic: 9093639571905543687,
        shift: 49,
    },
    MagicHasher {
        premask: 420017753620736,
        magic: 11966717751162054848,
        shift: 50,
    },
    MagicHasher {
        premask: 699298018886144,
        magic: 13252543049725546200,
        shift: 50,
    },
    MagicHasher {
        premask: 1260057572672512,
        magic: 840624458771477143,
        shift: 50,
    },
    MagicHasher {
        premask: 2381576680245248,
        magic: 16830428796971213850,
        shift: 50,
    },
    MagicHasher {
        premask: 4624614895390720,
        magic: 9740248168835649851,
        shift: 50,
    },
    MagicHasher {
        premask: 9110691325681664,
        magic: 13568054090654919574,
        shift: 50,
    },
    MagicHasher {
        premask: 18082844186263552,
        magic: 7273760038836957738,
        shift: 51,
    },
    MagicHasher {
        premask: 36167887395782656,
        magic: 13368937669876883566,
        shift: 49,
    },
    MagicHasher {
        premask: 35466950888980736,
        magic: 17926798989389257057,
        shift: 50,
    },
    MagicHasher {
        premask: 34905104758997504,
        magic: 11170633369579735213,
        shift: 51,
    },
    MagicHasher {
        premask: 34344362452452352,
        magic: 10675036498041528164,
        shift: 51,
    },
    MagicHasher {
        premask: 33222877839362048,
        magic: 4330412293351519626,
        shift: 51,
    },
    MagicHasher {
        premask: 30979908613181440,
        magic: 9670307612112330716,
        shift: 51,
    },
    MagicHasher {
        premask: 26493970160820224,
        magic: 9579643926881410477,
        shift: 51,
    },
    MagicHasher {
        premask: 17522093256097792,
        magic: 16539288583617098105,
        shift: 51,
    },
    MagicHasher {
        premask: 35607136465616896,
        magic: 16088674110749848818,
        shift: 50,
    },
    MagicHasher {
        premask: 9079539427579068672,
        magic: 16392743913835888970,
        shift: 49,
    },
    MagicHasher {
        premask: 8935706818303361536,
        magic: 2121985063587448486,
        shift: 50,
    },
    MagicHasher {
        premask: 8792156787827803136,
        magic: 14735840970564610215,
        shift: 50,
    },
    MagicHasher {
        premask: 8505056726876686336,
        magic: 10458496305231550750,
        shift: 50,
    },
    MagicHasher {
        premask: 7930856604974452736,
        magic: 14744112799991328191,
        shift: 50,
    },
    MagicHasher {
        premask: 6782456361169985536,
        magic: 12328886626809718821,
        shift: 49,
    },
    MagicHasher {
        premask: 4485655873561051136,
        magic: 18010442351099170677,
        shift: 50,
    },
    MagicHasher {
        premask: 9115426935197958144,
        magic: 15798047004492293086,
        shift: 49,
    },
];

pub const BISHOP_MAGIC_HASHERS: [MagicHasher; 64] = [
    MagicHasher {
        premask: 18049651735527936,
        magic: 11036326894813940552,
        shift: 57,
    },
    MagicHasher {
        premask: 70506452091904,
        magic: 13584494337795827194,
        shift: 58,
    },
    MagicHasher {
        premask: 275415828992,
        magic: 11635047460545371872,
        shift: 57,
    },
    MagicHasher {
        premask: 1075975168,
        magic: 8550759138699398471,
        shift: 57,
    },
    MagicHasher {
        premask: 38021120,
        magic: 15458215649374692618,
        shift: 57,
    },
    MagicHasher {
        premask: 8657588224,
        magic: 1756604031713676995,
        shift: 57,
    },
    MagicHasher {
        premask: 2216338399232,
        magic: 8123920126173607456,
        shift: 58,
    },
    MagicHasher {
        premask: 567382630219776,
        magic: 13260278408954650437,
        shift: 57,
    },
    MagicHasher {
        premask: 9024825867763712,
        magic: 12329829513490647303,
        shift: 58,
    },
    MagicHasher {
        premask: 18049651735527424,
        magic: 2299564057036543316,
        shift: 58,
    },
    MagicHasher {
        premask: 70506452221952,
        magic: 9446341484559371685,
        shift: 58,
    },
    MagicHasher {
        premask: 275449643008,
        magic: 10409722730494822733,
        shift: 57,
    },
    MagicHasher {
        premask: 9733406720,
        magic: 148730670711410283,
        shift: 57,
    },
    MagicHasher {
        premask: 2216342585344,
        magic: 12970224151283547509,
        shift: 57,
    },
    MagicHasher {
        premask: 567382630203392,
        magic: 3197114751172490835,
        shift: 58,
    },
    MagicHasher {
        premask: 1134765260406784,
        magic: 16897577946514929643,
        shift: 58,
    },
    MagicHasher {
        premask: 4512412933816832,
        magic: 4089300214968240247,
        shift: 58,
    },
    MagicHasher {
        premask: 9024825867633664,
        magic: 13874762536028388503,
        shift: 58,
    },
    MagicHasher {
        premask: 18049651768822272,
        magic: 17927335857030250667,
        shift: 55,
    },
    MagicHasher {
        premask: 70515108615168,
        magic: 10304918797361639559,
        shift: 55,
    },
    MagicHasher {
        premask: 2491752130560,
        magic: 9125744411401724785,
        shift: 55,
    },
    MagicHasher {
        premask: 567383701868544,
        magic: 5447915233389447439,
        shift: 55,
    },
    MagicHasher {
        premask: 1134765256220672,
        magic: 12678518074338450840,
        shift: 58,
    },
    MagicHasher {
        premask: 2269530512441344,
        magic: 9011249559655360979,
        shift: 58,
    },
    MagicHasher {
        premask: 2256206450263040,
        magic: 3886742031205317250,
        shift: 57,
    },
    MagicHasher {
        premask: 4512412900526080,
        magic: 13562812133884349812,
        shift: 57,
    },
    MagicHasher {
        premask: 9024834391117824,
        magic: 5110873445040553228,
        shift: 55,
    },
    MagicHasher {
        premask: 18051867805491712,
        magic: 10680463215617409027,
        shift: 53,
    },
    MagicHasher {
        premask: 637888545440768,
        magic: 9777059234664143070,
        shift: 53,
    },
    MagicHasher {
        premask: 1135039602493440,
        magic: 10976096459216277482,
        shift: 55,
    },
    MagicHasher {
        premask: 2269529440784384,
        magic: 1906204496909866544,
        shift: 57,
    },
    MagicHasher {
        premask: 4539058881568768,
        magic: 9390810690003615257,
        shift: 57,
    },
    MagicHasher {
        premask: 1128098963916800,
        magic: 6846384432671626567,
        shift: 57,
    },
    MagicHasher {
        premask: 2256197927833600,
        magic: 17452001451584850951,
        shift: 57,
    },
    MagicHasher {
        premask: 4514594912477184,
        magic: 677908901203596788,
        shift: 55,
    },
    MagicHasher {
        premask: 9592139778506752,
        magic: 14719071464164980259,
        shift: 53,
    },
    MagicHasher {
        premask: 19184279556981248,
        magic: 2360496266827912778,
        shift: 53,
    },
    MagicHasher {
        premask: 2339762086609920,
        magic: 7189877777650640648,
        shift: 55,
    },
    MagicHasher {
        premask: 4538784537380864,
        magic: 9965189106034913128,
        shift: 58,
    },
    MagicHasher {
        premask: 9077569074761728,
        magic: 13807539491698880248,
        shift: 57,
    },
    MagicHasher {
        premask: 562958610993152,
        magic: 9931155789059899645,
        shift: 58,
    },
    MagicHasher {
        premask: 1125917221986304,
        magic: 5181650230270888192,
        shift: 58,
    },
    MagicHasher {
        premask: 2814792987328512,
        magic: 15279943646091604102,
        shift: 55,
    },
    MagicHasher {
        premask: 5629586008178688,
        magic: 6349022349102919842,
        shift: 55,
    },
    MagicHasher {
        premask: 11259172008099840,
        magic: 11889196303829189738,
        shift: 55,
    },
    MagicHasher {
        premask: 22518341868716544,
        magic: 904921724660586768,
        shift: 55,
    },
    MagicHasher {
        premask: 9007336962655232,
        magic: 7206574785518236356,
        shift: 58,
    },
    MagicHasher {
        premask: 18014673925310464,
        magic: 3653548434665420755,
        shift: 58,
    },
    MagicHasher {
        premask: 2216338399232,
        magic: 550395524403285798,
        shift: 58,
    },
    MagicHasher {
        premask: 4432676798464,
        magic: 7169507115524710642,
        shift: 58,
    },
    MagicHasher {
        premask: 11064376819712,
        magic: 2433904800380960324,
        shift: 57,
    },
    MagicHasher {
        premask: 22137335185408,
        magic: 10026403844186396490,
        shift: 57,
    },
    MagicHasher {
        premask: 44272556441600,
        magic: 11087325473644297768,
        shift: 57,
    },
    MagicHasher {
        premask: 87995357200384,
        magic: 3109639689236617605,
        shift: 57,
    },
    MagicHasher {
        premask: 35253226045952,
        magic: 16423648380411026962,
        shift: 58,
    },
    MagicHasher {
        premask: 70506452091904,
        magic: 9249383812535209505,
        shift: 58,
    },
    MagicHasher {
        premask: 567382630219776,
        magic: 6945120446219581182,
        shift: 57,
    },
    MagicHasher {
        premask: 1134765260406784,
        magic: 4876368492743762540,
        shift: 58,
    },
    MagicHasher {
        premask: 2832480465846272,
        magic: 15756126214970558773,
        shift: 57,
    },
    MagicHasher {
        premask: 5667157807464448,
        magic: 11242517333640285750,
        shift: 57,
    },
    MagicHasher {
        premask: 11333774449049600,
        magic: 4094650471627271205,
        shift: 57,
    },
    MagicHasher {
        premask: 22526811443298304,
        magic: 15770214442173541990,
        shift: 57,
    },
    MagicHasher {
        premask: 9024825867763712,
        magic: 12819715631596370710,
        shift: 58,
    },
    MagicHasher {
        premask: 18049651735527936,
        magic: 17373860460098386682,
        shift: 57,
    },
];

// pub const ROOK_MAGIC_HASHERS: [MagicHasher; 64] = [
//     MagicHasher {
//         premask: 282578800148862,
//         magic: 12393882513112026140,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 565157600297596,
//         magic: 16300536615448215550,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 1130315200595066,
//         magic: 9431097393599463424,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 2260630401190006,
//         magic: 5540797841960591360,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 4521260802379886,
//         magic: 7926364129043144705,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 9042521604759646,
//         magic: 9691798186884073472,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 18085043209519166,
//         magic: 13790117757692214272,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 36170086419038334,
//         magic: 723590819259047424,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 282578800180736,
//         magic: 4164703899296597284,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 565157600328704,
//         magic: 724551884886048760,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 1130315200625152,
//         magic: 12863125051431666433,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 2260630401218048,
//         magic: 4096586862976434172,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 4521260802403840,
//         magic: 8420042517646408993,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 9042521604775424,
//         magic: 8455560589070028801,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 18085043209518592,
//         magic: 17144078082013396877,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 36170086419037696,
//         magic: 10390367297417448924,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 282578808340736,
//         magic: 10164875046788267906,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 565157608292864,
//         magic: 2482040263227261339,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 1130315208328192,
//         magic: 18058557096286355842,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 2260630408398848,
//         magic: 6682745912013363713,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 4521260808540160,
//         magic: 13468826125535293953,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 9042521608822784,
//         magic: 10611055160617702032,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 18085043209388032,
//         magic: 17767584537420769290,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 36170086418907136,
//         magic: 18220400809065320277,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 282580897300736,
//         magic: 6687967543972211639,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 565159647117824,
//         magic: 2344551376201318465,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 1130317180306432,
//         magic: 13422632479365242956,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 2260632246683648,
//         magic: 14741496629853405285,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 4521262379438080,
//         magic: 18291377108039020626,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 9042522644946944,
//         magic: 4402815333564416865,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 18085043175964672,
//         magic: 4614907845197434983,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 36170086385483776,
//         magic: 13450837298958666765,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 283115671060736,
//         magic: 7317688161780353748,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 565681586307584,
//         magic: 17736436826450264254,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 1130822006735872,
//         magic: 15875611869642248256,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 2261102847592448,
//         magic: 6907071372919752043,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 4521664529305600,
//         magic: 8529128498564953876,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 9042787892731904,
//         magic: 5182521943789668955,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 18085034619584512,
//         magic: 17445038258271782881,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 36170077829103616,
//         magic: 1162558786460705081,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 420017753620736,
//         magic: 5334455153081559900,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 699298018886144,
//         magic: 10944169556394554419,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 1260057572672512,
//         magic: 9690766393682492679,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 2381576680245248,
//         magic: 5670019441515688490,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 4624614895390720,
//         magic: 13764970747231374109,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 9110691325681664,
//         magic: 8766725104060705281,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 18082844186263552,
//         magic: 9757764282873560930,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 36167887395782656,
//         magic: 16710162559125369229,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 35466950888980736,
//         magic: 7791134854616239104,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 34905104758997504,
//         magic: 7926329556047165904,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 34344362452452352,
//         magic: 13473364088356958976,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 33222877839362048,
//         magic: 6557239468353762816,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 30979908613181440,
//         magic: 12033618006761920000,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 26493970160820224,
//         magic: 12114120288466900480,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 17522093256097792,
//         magic: 7083168736482374656,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 35607136465616896,
//         magic: 1020723420621721088,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 9079539427579068672,
//         magic: 520938430754667148,
//         shift: 52,
//     },
//     MagicHasher {
//         premask: 8935706818303361536,
//         magic: 5872691426220854398,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 8792156787827803136,
//         magic: 12213762225165068490,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 8505056726876686336,
//         magic: 16429131068009480110,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 7930856604974452736,
//         magic: 7174797260443623482,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 6782456361169985536,
//         magic: 13781014878279116802,
//         shift: 53,
//     },
//     MagicHasher {
//         premask: 4485655873561051136,
//         magic: 3223451510870575148,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 9115426935197958144,
//         magic: 16570994835554228494,
//         shift: 53,
//     },
// ];
//
// pub const BISHOP_MAGIC_HASHERS: [MagicHasher; 64] = [
//     MagicHasher {
//         premask: 18049651735527936,
//         magic: 6791474864611757060,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 70506452091904,
//         magic: 7499112530489114448,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 275415828992,
//         magic: 8622271177600044493,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 1075975168,
//         magic: 4469827659624412402,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 38021120,
//         magic: 2813067585713127408,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 8657588224,
//         magic: 2271653026214686767,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 2216338399232,
//         magic: 1149542245494416309,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 567382630219776,
//         magic: 16628976621147062280,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 9024825867763712,
//         magic: 3036845381962635256,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 18049651735527424,
//         magic: 10990688979381298174,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 70506452221952,
//         magic: 7723348419871505200,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 275449643008,
//         magic: 6241786816352721112,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 9733406720,
//         magic: 17419384675554449910,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 2216342585344,
//         magic: 5337629797065435516,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 567382630203392,
//         magic: 7528990312611872434,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 1134765260406784,
//         magic: 4831448365257736157,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 4512412933816832,
//         magic: 4449600180738183149,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 9024825867633664,
//         magic: 9340486265188227049,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 18049651768822272,
//         magic: 887211635167785186,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 70515108615168,
//         magic: 9544256224793467780,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 2491752130560,
//         magic: 3147454065829610951,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 567383701868544,
//         magic: 4529073186046386177,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 1134765256220672,
//         magic: 16308664803584802627,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 2269530512441344,
//         magic: 7154531282928214007,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 2256206450263040,
//         magic: 7941020453771886550,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 4512412900526080,
//         magic: 17982833550070901608,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 9024834391117824,
//         magic: 3226837982187333601,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 18051867805491712,
//         magic: 8997191253756682568,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 637888545440768,
//         magic: 18081824963186155530,
//         shift: 55,
//     },
//     MagicHasher {
//         premask: 1135039602493440,
//         magic: 7875817186126873096,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 2269529440784384,
//         magic: 9482350501171344385,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 4539058881568768,
//         magic: 16239430089774489560,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 1128098963916800,
//         magic: 9324769480510895688,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 2256197927833600,
//         magic: 4539923232720836613,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 4514594912477184,
//         magic: 844416220665086977,
//         shift: 57,
//     },
//     MagicHasher {
//         premask: 9592139778506752,
//         magic: 14928089520601402467,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 19184279556981248,
//         magic: 4356111086088622146,
//         shift: 54,
//     },
//     MagicHasher {
//         premask: 2339762086609920,
//         magic: 12377571792544000818,
//         shift: 56,
//     },
//     MagicHasher {
//         premask: 4538784537380864,
//         magic: 13768186248078761218,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 9077569074761728,
//         magic: 2896716498385559933,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 562958610993152,
//         magic: 7318317858568518858,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 1125917221986304,
//         magic: 13338101209057593194,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 2814792987328512,
//         magic: 451117367685367504,
//         shift: 56,
//     },
//     MagicHasher {
//         premask: 5629586008178688,
//         magic: 4399738569223932336,
//         shift: 56,
//     },
//     MagicHasher {
//         premask: 11259172008099840,
//         magic: 12815110503711155628,
//         shift: 56,
//     },
//     MagicHasher {
//         premask: 22518341868716544,
//         magic: 12737021131892731929,
//         shift: 56,
//     },
//     MagicHasher {
//         premask: 9007336962655232,
//         magic: 9223257213191261196,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 18014673925310464,
//         magic: 17239664914785975212,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 2216338399232,
//         magic: 1105632240994323946,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 4432676798464,
//         magic: 15617356390490049760,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 11064376819712,
//         magic: 14471330052756067262,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 22137335185408,
//         magic: 9774271290522035875,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 44272556441600,
//         magic: 18182382925105383297,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 87995357200384,
//         magic: 16768221405807302376,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 35253226045952,
//         magic: 10030303534279217359,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 70506452091904,
//         magic: 7782132755381280109,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 567382630219776,
//         magic: 1371911262438699025,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 1134765260406784,
//         magic: 9065970347511490345,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 2832480465846272,
//         magic: 5675639115175713521,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 5667157807464448,
//         magic: 3896404074769458234,
//         shift: 58,
//     },
//     MagicHasher {
//         premask: 11333774449049600,
//         magic: 5936855253689467265,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 22526811443298304,
//         magic: 6405295973796135651,
//         shift: 59,
//     },
//     MagicHasher {
//         premask: 9024825867763712,
//         magic: 9088404355728160208,
//         shift: 60,
//     },
//     MagicHasher {
//         premask: 18049651735527936,
//         magic: 4828193461272579640,
//         shift: 59,
//     },
// ];
