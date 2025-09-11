use crate::magic_bitboards::MagicHasher;

pub const ROOK_MAGIC_HASHERS: [MagicHasher; 64] = [
    MagicHasher {
        premask: 282578800148862,
        magic: 17320757513885329380,
        shift: 51,
    },
    MagicHasher {
        premask: 565157600297596,
        magic: 14040013259316833648,
        shift: 52,
    },
    MagicHasher {
        premask: 1130315200595066,
        magic: 3390951326127643456,
        shift: 52,
    },
    MagicHasher {
        premask: 2260630401190006,
        magic: 11528907474292928376,
        shift: 52,
    },
    MagicHasher {
        premask: 4521260802379886,
        magic: 14686482375561331759,
        shift: 52,
    },
    MagicHasher {
        premask: 9042521604759646,
        magic: 5604719001789413378,
        shift: 52,
    },
    MagicHasher {
        premask: 18085043209519166,
        magic: 8699149831850574947,
        shift: 52,
    },
    MagicHasher {
        premask: 36170086419038334,
        magic: 11047324417693582315,
        shift: 51,
    },
    MagicHasher {
        premask: 282578800180736,
        magic: 6756738628415217918,
        shift: 52,
    },
    MagicHasher {
        premask: 565157600328704,
        magic: 18033419929655826498,
        shift: 53,
    },
    MagicHasher {
        premask: 1130315200625152,
        magic: 9483455482326223381,
        shift: 53,
    },
    MagicHasher {
        premask: 2260630401218048,
        magic: 9547527235827710013,
        shift: 53,
    },
    MagicHasher {
        premask: 4521260802403840,
        magic: 13418950453300131408,
        shift: 53,
    },
    MagicHasher {
        premask: 9042521604775424,
        magic: 3446062961240019680,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043209518592,
        magic: 14228286600742362156,
        shift: 53,
    },
    MagicHasher {
        premask: 36170086419037696,
        magic: 11613958831604768422,
        shift: 52,
    },
    MagicHasher {
        premask: 282578808340736,
        magic: 16949634225932225271,
        shift: 52,
    },
    MagicHasher {
        premask: 565157608292864,
        magic: 16391336522771540585,
        shift: 53,
    },
    MagicHasher {
        premask: 1130315208328192,
        magic: 15860538741817720403,
        shift: 53,
    },
    MagicHasher {
        premask: 2260630408398848,
        magic: 18439371588346071623,
        shift: 53,
    },
    MagicHasher {
        premask: 4521260808540160,
        magic: 4337493140497213842,
        shift: 53,
    },
    MagicHasher {
        premask: 9042521608822784,
        magic: 9940814090578226623,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043209388032,
        magic: 6646237669503780903,
        shift: 53,
    },
    MagicHasher {
        premask: 36170086418907136,
        magic: 18113214599986771480,
        shift: 52,
    },
    MagicHasher {
        premask: 282580897300736,
        magic: 871950246499151818,
        shift: 52,
    },
    MagicHasher {
        premask: 565159647117824,
        magic: 3791983435856191685,
        shift: 53,
    },
    MagicHasher {
        premask: 1130317180306432,
        magic: 5458678415099539004,
        shift: 53,
    },
    MagicHasher {
        premask: 2260632246683648,
        magic: 8294850406147807645,
        shift: 53,
    },
    MagicHasher {
        premask: 4521262379438080,
        magic: 6839344473795618220,
        shift: 53,
    },
    MagicHasher {
        premask: 9042522644946944,
        magic: 9478627716812156995,
        shift: 53,
    },
    MagicHasher {
        premask: 18085043175964672,
        magic: 5894738435104472955,
        shift: 53,
    },
    MagicHasher {
        premask: 36170086385483776,
        magic: 13490913200405914368,
        shift: 52,
    },
    MagicHasher {
        premask: 283115671060736,
        magic: 1415705587904296677,
        shift: 52,
    },
    MagicHasher {
        premask: 565681586307584,
        magic: 14030862133726338005,
        shift: 53,
    },
    MagicHasher {
        premask: 1130822006735872,
        magic: 8441441463197664133,
        shift: 53,
    },
    MagicHasher {
        premask: 2261102847592448,
        magic: 281260604241814817,
        shift: 53,
    },
    MagicHasher {
        premask: 4521664529305600,
        magic: 10997759758892807708,
        shift: 53,
    },
    MagicHasher {
        premask: 9042787892731904,
        magic: 3076227511496689869,
        shift: 53,
    },
    MagicHasher {
        premask: 18085034619584512,
        magic: 933808207862699571,
        shift: 53,
    },
    MagicHasher {
        premask: 36170077829103616,
        magic: 9560673346762113188,
        shift: 52,
    },
    MagicHasher {
        premask: 420017753620736,
        magic: 8636980977133609028,
        shift: 52,
    },
    MagicHasher {
        premask: 699298018886144,
        magic: 11970852963020025804,
        shift: 53,
    },
    MagicHasher {
        premask: 1260057572672512,
        magic: 10787257181337051132,
        shift: 53,
    },
    MagicHasher {
        premask: 2381576680245248,
        magic: 7863069245169174815,
        shift: 53,
    },
    MagicHasher {
        premask: 4624614895390720,
        magic: 9043476030285676862,
        shift: 53,
    },
    MagicHasher {
        premask: 9110691325681664,
        magic: 15883695470757689635,
        shift: 53,
    },
    MagicHasher {
        premask: 18082844186263552,
        magic: 9767803506355746148,
        shift: 53,
    },
    MagicHasher {
        premask: 36167887395782656,
        magic: 3264637091039282857,
        shift: 52,
    },
    MagicHasher {
        premask: 35466950888980736,
        magic: 5130866629160724224,
        shift: 53,
    },
    MagicHasher {
        premask: 34905104758997504,
        magic: 2258352379091419648,
        shift: 54,
    },
    MagicHasher {
        premask: 34344362452452352,
        magic: 16861469456374040064,
        shift: 54,
    },
    MagicHasher {
        premask: 33222877839362048,
        magic: 13051431471936847360,
        shift: 54,
    },
    MagicHasher {
        premask: 30979908613181440,
        magic: 7359965909865684480,
        shift: 54,
    },
    MagicHasher {
        premask: 26493970160820224,
        magic: 9376494039985064992,
        shift: 54,
    },
    MagicHasher {
        premask: 17522093256097792,
        magic: 13059653904366572544,
        shift: 54,
    },
    MagicHasher {
        premask: 35607136465616896,
        magic: 16043230365545760256,
        shift: 53,
    },
    MagicHasher {
        premask: 9079539427579068672,
        magic: 6124895067340646206,
        shift: 52,
    },
    MagicHasher {
        premask: 8935706818303361536,
        magic: 16068843261420781586,
        shift: 53,
    },
    MagicHasher {
        premask: 8792156787827803136,
        magic: 16104869779769095846,
        shift: 53,
    },
    MagicHasher {
        premask: 8505056726876686336,
        magic: 405323660211987778,
        shift: 53,
    },
    MagicHasher {
        premask: 7930856604974452736,
        magic: 16380155655305053222,
        shift: 53,
    },
    MagicHasher {
        premask: 6782456361169985536,
        magic: 2374564289869517618,
        shift: 52,
    },
    MagicHasher {
        premask: 4485655873561051136,
        magic: 3796534201742586620,
        shift: 53,
    },
    MagicHasher {
        premask: 9115426935197958144,
        magic: 9384651753964437902,
        shift: 52,
    },
];

pub const BISHOP_MAGIC_HASHERS: [MagicHasher; 64] = [
    MagicHasher {
        premask: 18049651735527936,
        magic: 15514742657465454785,
        shift: 58,
    },
    MagicHasher {
        premask: 70506452091904,
        magic: 5183814800573725054,
        shift: 59,
    },
    MagicHasher {
        premask: 275415828992,
        magic: 14147575972036883382,
        shift: 58,
    },
    MagicHasher {
        premask: 1075975168,
        magic: 3731883656568028139,
        shift: 58,
    },
    MagicHasher {
        premask: 38021120,
        magic: 16545149479289191316,
        shift: 58,
    },
    MagicHasher {
        premask: 8657588224,
        magic: 2919970941078668916,
        shift: 58,
    },
    MagicHasher {
        premask: 2216338399232,
        magic: 1023205561886723840,
        shift: 59,
    },
    MagicHasher {
        premask: 567382630219776,
        magic: 12204004787589742586,
        shift: 58,
    },
    MagicHasher {
        premask: 9024825867763712,
        magic: 8412599212074082299,
        shift: 59,
    },
    MagicHasher {
        premask: 18049651735527424,
        magic: 7683730002545339658,
        shift: 59,
    },
    MagicHasher {
        premask: 70506452221952,
        magic: 3415453249314626947,
        shift: 58,
    },
    MagicHasher {
        premask: 275449643008,
        magic: 15076542518991174981,
        shift: 58,
    },
    MagicHasher {
        premask: 9733406720,
        magic: 12337284744962599565,
        shift: 58,
    },
    MagicHasher {
        premask: 2216342585344,
        magic: 1327615829094212456,
        shift: 58,
    },
    MagicHasher {
        premask: 567382630203392,
        magic: 13989988554989010892,
        shift: 59,
    },
    MagicHasher {
        premask: 1134765260406784,
        magic: 3860021294669250451,
        shift: 59,
    },
    MagicHasher {
        premask: 4512412933816832,
        magic: 15717594500210331634,
        shift: 59,
    },
    MagicHasher {
        premask: 9024825867633664,
        magic: 12290341115420617090,
        shift: 59,
    },
    MagicHasher {
        premask: 18049651768822272,
        magic: 12684036716520718090,
        shift: 56,
    },
    MagicHasher {
        premask: 70515108615168,
        magic: 8236657713472324044,
        shift: 56,
    },
    MagicHasher {
        premask: 2491752130560,
        magic: 6271193209804237111,
        shift: 56,
    },
    MagicHasher {
        premask: 567383701868544,
        magic: 6850048220029643605,
        shift: 56,
    },
    MagicHasher {
        premask: 1134765256220672,
        magic: 13989249352649961536,
        shift: 59,
    },
    MagicHasher {
        premask: 2269530512441344,
        magic: 14305973585720583431,
        shift: 59,
    },
    MagicHasher {
        premask: 2256206450263040,
        magic: 11433603480215797923,
        shift: 58,
    },
    MagicHasher {
        premask: 4512412900526080,
        magic: 12489756278438849945,
        shift: 58,
    },
    MagicHasher {
        premask: 9024834391117824,
        magic: 5543620558769898231,
        shift: 56,
    },
    MagicHasher {
        premask: 18051867805491712,
        magic: 4722274408389847895,
        shift: 54,
    },
    MagicHasher {
        premask: 637888545440768,
        magic: 98419900671290694,
        shift: 54,
    },
    MagicHasher {
        premask: 1135039602493440,
        magic: 5154016159006121596,
        shift: 56,
    },
    MagicHasher {
        premask: 2269529440784384,
        magic: 1237896659479165654,
        shift: 58,
    },
    MagicHasher {
        premask: 4539058881568768,
        magic: 2010400308935509369,
        shift: 58,
    },
    MagicHasher {
        premask: 1128098963916800,
        magic: 6233328493871812943,
        shift: 58,
    },
    MagicHasher {
        premask: 2256197927833600,
        magic: 3911460636118419064,
        shift: 58,
    },
    MagicHasher {
        premask: 4514594912477184,
        magic: 1838752725653819537,
        shift: 56,
    },
    MagicHasher {
        premask: 9592139778506752,
        magic: 18299583923421775199,
        shift: 54,
    },
    MagicHasher {
        premask: 19184279556981248,
        magic: 18189865416450162156,
        shift: 54,
    },
    MagicHasher {
        premask: 2339762086609920,
        magic: 12964092639603717673,
        shift: 56,
    },
    MagicHasher {
        premask: 4538784537380864,
        magic: 3759417851011563838,
        shift: 58,
    },
    MagicHasher {
        premask: 9077569074761728,
        magic: 2277910665398892349,
        shift: 58,
    },
    MagicHasher {
        premask: 562958610993152,
        magic: 7043520842669888870,
        shift: 59,
    },
    MagicHasher {
        premask: 1125917221986304,
        magic: 3558415597832913551,
        shift: 59,
    },
    MagicHasher {
        premask: 2814792987328512,
        magic: 7058068592254358738,
        shift: 56,
    },
    MagicHasher {
        premask: 5629586008178688,
        magic: 1364250042772074115,
        shift: 56,
    },
    MagicHasher {
        premask: 11259172008099840,
        magic: 15571108784841172041,
        shift: 56,
    },
    MagicHasher {
        premask: 22518341868716544,
        magic: 7623771908489059710,
        shift: 56,
    },
    MagicHasher {
        premask: 9007336962655232,
        magic: 13510660098533423299,
        shift: 59,
    },
    MagicHasher {
        premask: 18014673925310464,
        magic: 5395272623407267619,
        shift: 59,
    },
    MagicHasher {
        premask: 2216338399232,
        magic: 6235232151113953925,
        shift: 59,
    },
    MagicHasher {
        premask: 4432676798464,
        magic: 3047482816036424470,
        shift: 59,
    },
    MagicHasher {
        premask: 11064376819712,
        magic: 12838045819297196834,
        shift: 58,
    },
    MagicHasher {
        premask: 22137335185408,
        magic: 1075774682513102638,
        shift: 58,
    },
    MagicHasher {
        premask: 44272556441600,
        magic: 12288855597731918557,
        shift: 58,
    },
    MagicHasher {
        premask: 87995357200384,
        magic: 7630957203943736813,
        shift: 58,
    },
    MagicHasher {
        premask: 35253226045952,
        magic: 18266581819069295512,
        shift: 59,
    },
    MagicHasher {
        premask: 70506452091904,
        magic: 7178755141233660804,
        shift: 59,
    },
    MagicHasher {
        premask: 567382630219776,
        magic: 4141058203347666517,
        shift: 58,
    },
    MagicHasher {
        premask: 1134765260406784,
        magic: 1706688328464380812,
        shift: 59,
    },
    MagicHasher {
        premask: 2832480465846272,
        magic: 18139508229602344104,
        shift: 58,
    },
    MagicHasher {
        premask: 5667157807464448,
        magic: 7795978534936867411,
        shift: 58,
    },
    MagicHasher {
        premask: 11333774449049600,
        magic: 8167620339974406278,
        shift: 58,
    },
    MagicHasher {
        premask: 22526811443298304,
        magic: 17123665445026313152,
        shift: 58,
    },
    MagicHasher {
        premask: 9024825867763712,
        magic: 1035546218440905388,
        shift: 59,
    },
    MagicHasher {
        premask: 18049651735527936,
        magic: 8971115372103014892,
        shift: 58,
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
