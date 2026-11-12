# Trails through Daybreak II: персонажи и их модели

Все записи `t_name.tbl` этой игры, английский. Полное имя — как написано
в таблице; если полного имени нет — имя.

- **ID:** номер персонажа в `t_name.tbl`, его указывают скрипты, например
  `system[5,6](106, ...)`. Это не номер модели. `65535`, похоже, значит, что своего ID
  у записи нет; скрипты находят такие модели по имени.
- **CHR:** файл модели. Маска для извлечения из архивов: номер модели и звёздочка,
  например `chr0000*`.
- **Портрет:** портрет, который CalvardCompare показывает у этого ID: `avfc` — по грудь,
  `btlface` — голова. Пусто — портрета нет.

ID    | CHR             | Полное имя в игре | Портрет
----: | --------------- | --------------------- | ----
0     | chr0000         | Van Arkride | avfc0000
1     | chr5001_c10     | Agnès Claudel | avfc5001_c10
2     | chr5003         | Feri Al-Fayed | avfc5003
3     | chr0002         | Aaron Wei | avfc0002
4     | chr0004         | Quatre Salision | avfc0004
5     | chr0006         | Bergard Zeman | avfc0006
6     | chr5005         | Risette Twinings | avfc5005
7     | chr0113         | Walter Kron | avfc0113
8     | chr0111         | Zin Vathek | avfc0111
9     | chr0112         | René Kincaid | avfc0112
9     | chr0112_c02     | René Kincaid | avfc0112
10    | chr5007         | Judith Lanster | avfc5007
11    | chr5110_c10     | Elaine Auclair | avfc5110
11    | chr5110         | Elaine (Bracer Outfit) | avfc5110
12    | chr5117_c10     | Shizuna Rem Misurugi | avfc5117_c10
13    | chr0304         | Kurogane | avfc0304
14    | chr5118         | Lucrezia Isselee | avfc5118
15    | chr0009         | Grendel Van | avfc0009
17    | chr0120         | Swin Abel | avfc0120
17    | chr0120_c02     | Swin_Towel | avfc0120
18    | chr5119         | Nadia Rayne | avfc5119
18    | chr5119_c02     | Nadia_Towel | avfc5119
19    | chr5114_c10     | Renne Bright | avfc5114_c10
62    | chr5113         | Rixia Mao | avfc5113
63    | chr5112         | Fie Claussell | avfc5112
64    | rob0000         | FIO
64    | rob0000_c00     | FIO_Electric World
64    | rob0000_c01     | FIO_Electric World (Yellow)
65    | rob0001         | XEROS
100   | chr0114         | Alexandre | btlface0_c0114
101   | chr0115         | Melchior | btlface0_c0115
102   | chr0116         | Gerard Dantès | btlface0_c0116
103   | chr0117         | Arioch | btlface0_c0117
104   | chr0118         | Ellroy Harwood | btlface0_c0118
105   | chr0119         | President Gramheart
106   | chr0300         | Alvis | btlface0_c0300
107   | chr0301         | Cao Lee | avfc0301
108   | chr0302         | Sherid Asval | btlface0_c0302
110   | chr5115         | Viola | btlface0_c5115
111   | chr5116         | Olympia | btlface0_c5116
112   | chr5300         | Naje Berka | btlface0_c5300
113   | chr5301         | Aida | btlface0_c5301
114   | chr5310         | Paulette
114   | chr5310_c00     | Paulette (Different Outfit)
114   | chr5503         | Paulette
115   | chr0310         | Victor
116   | chr5311         | Yumé | btlface0_c5311
117   | chr0311         | Giacomo
117   | chr0311_c00     | Giacomo (Corpse Version)
118   | chr0315         | Detective Nate
119   | chr0314         | Inspector Daswani
120   | chr5320         | Second Lieutenant Kaela | btlface0_c5320
121   | chr5313         | Marielle | btlface0_c5313
121   | chr5313_c00     | Marielle (No Hat) | btlface0_c5313
121   | chr5313_c01     | Marielle (Clothes Half-Disintegrated) | btlface0_c5313
121   | chr5313_c02     | Marielle (Clothes Half-Disintegrated/No Slime) | btlface0_c5313
122   | chr0316         | Dingo | btlface0_c0316
123   | chr0313         | Bermotti
124   | chr0329         | Commander Geraint | btlface0_c0329
125   | chr0328         | Vice Chief Hasan | btlface0_c0328
126   | chr5322         | Kilika | btlface0_c5322
127   | chr5312         | Odette
127   | chr5312_c12     | Odette (Jersey)
128   | chr0312         | Albert
129   | chr5315         | Nina | btlface0_c5315
130   | chr0318         | Maxim | btlface0_c0318
130   | chr0318_c00     | Maxim (Muddy) | btlface0_c0318
131   | chr5316         | Ashen
132   | chr0320         | Fan
133   | chr0319         | Gien
133   | chr0319_c00     | Gien (Hawaiian Shirt)
134   | chr0317         | Jack | btlface0_c0317
135   | chr5314         | Halle | btlface0_c5314
136   | chr5317         | Saara
136   | chr5317_c04     | Saara (Pole Dancer)
137   | chr5318         | Shaheena
138   | chr0322         | Director Gotti
139   | chr0321         | President Dillon
139   | chr0321_c00     | President Dillon (Corpse Version)
140   | chr5321         | Professor Hamilton
141   | chr5319         | Associate Professor Esmeray | btlface0_c5319
141   | chr5319_c00     | Esmeray Archette (Resort Clothes) | btlface0_c5319
142   | chr0324         | Professor Cronkite
143   | chr0325         | Chief Engineer Giscard
144   | chr0326_c00     | Tausend
145   | chr0323         | Professor Callaghan
146   | chr0331         | Rocksmith
147   | chr0330         | Warrant Officer Cody | btlface0_c0330_c00
147   | chr0330_c00     | Warrant Officer Cody_Special Ops Uniform | btlface0_c0330_c00
148   | chr0327         | President Auclair
149   | chr0303         | Kasim Al-Fayed | avfc0303
150   | chr0332         | Mr. Thorndyke
151   | chr5302         | Celis Ortesia | avfc5302
151   | chr5302_c02     | Celis Ortesia | avfc5302
152   | chr0306         | Leon Balthazar | avfc0306
152   | chr0306_c02     | Leon Balthazar | avfc0306
152   | chr0306_c03     | Leon Balthazar | avfc0306
153   | chr0305_c01     | Vice Captain Ashladd | btlface0_c0305_c01
154   | rob0010         | Ishtanti
154   | rob0010_c00     | Ishtanti_Cracked Face
154   | rob0010_c01     | Ishtanti_Damaged Version
154   | rob0010_c02     | Ishtanti_Berserk Version
155   | chr0010         | Grendel Zolga | btlface0_c0010
158   | chr0122         | Ixs | btlface0_c0122
158   | chr0122_c00     | Ixs (Twin Enforcer - Brother/Hooded) | btlface0_c0122
159   | chr5120         | Jorda | btlface0_c5120
159   | chr5120_c00     | Jorda (Twin Enforcer - Sister/Hooded) | btlface0_c5120
160   | chr0334         | Gaolang | btlface0_c0334
161   | chr0121         | Gardenmaster | btlface0_c0121
162   | etc0300         | Jorda's Shadow Hand
163   | chr0333         | Man in a Black Coat
164   | chr5324         | Princess Zita | btlface0_c5324
165   | chr5325         | Mirabel
165   | chr5325_c00     | Mirabel (No Headset)
166   | chr5325         | Mirabel
167   | chr5326         | Dominique
168   | chr5323         | Chief Developer Rosalie
169   | chr5327         | Towa | btlface0_c5327
172   | chr0122_c01     | Ixs (Disguised)
200   | chr5008         | Mare | avfc5008
201   | screen_aim      | Aim
202   | screen_vine     | Vine
203   | screen_sallos   | Sallos
204   | screen_camio    | Camio
205   | screen_carabia  | Carabia
206   | screen_sitri    | Sitri
207   | screen_otis     | Otis
208   | screen_raym     | Raym
209   | screen_pocel    | Pocel
210   | screen_gaap     | Gaap
211   | screen_oze      | Oze
212   | screen_loray    | Loray
213   | screen_nagi     | Nagi
214   | screen_bathym   | Bathym
215   | chr5009_c00     | Lapis | avfc5009_00
215   | screen_lapis    | Lapis | avfc5009_00
216   | screen_flau     | Frau
217   | screen_yakumo   | Yakumo
218   | screen_ronwe    | Ronwe
219   | screen_patel    | Pater-Mater
220   | chr5009         | Carmine Altera | avfc5009
225   | screen_kasim    | Voice Mod (Kasim)
226   | screen_nina     | Voice Mod (Nina)
227   | screen_cellis   | Voice Mod (Celis)
228   | screen_rufus    | Voice Mod (Surveillance Subject R.A.)
229   | screen_towa     | Voice Mod (Towa)
230   | screen_missy    | Voice Mod (Mishy SiN)
231   | screen_hamilton | Voice Mod (Young Hamilton)
232   | screen_renne    | Voice Mod (Renne)
233   | screen_judith   | Voice Mod (Actress Judith)
234   | screen_barkhorn | Voice Mod (Roaring Lion Barkhorn)
550   | chr0805_c08     | Male Student
551   | chr5700_c09     | Female Student
5000  | chr0785         | Gangster A | btlface0_c0785
5001  | chr0786         | Gangster B | btlface0_c0786
5002  | chr0787         | Gangster C | btlface0_c0787
5003  | chr0726         | Mafioso A | btlface0_c0726
5004  | chr0726_c01     | Mafioso B | btlface0_c0726_c01
5100  | chr0550         | Sei
5101  | chr0414         | Syd
5102  | chr0551_c01     | Huang
5103  | chr0560         | Antique Seller Jigen
5104  | chr0561         | Street Vendor Guhan
5105  | chr0550a        | Assistant Shenli | btlface0_c0550a
5106  | chr0847         | Eastern Medicine Dr. Longkai
5150  | chr0671         | Park Ranger
5151  | chr5611_c01     | Proprietress Hala
5152  | chr0830         | Bracer Renton
5153  | chr0836         | Bracer Ismail
5154  | chr0611a        | Manager Jahm
5155  | chr0630a        | Timur
5156  | chr0409         | Director Lam | btlface0_c0409
5250  | chr0715         | Mr. Powell
5251  | chr0805_c01     | Lyle
5252  | chr5700         | Chignon
5253  | chr0805         | Julian
5254  | chr0805_c02     | James
5255  | chr5700_c01     | Sasha
5256  | chr5680_c01     | Receptionist Trity
5257  | chr5752         | Bracer Blanca
5258  | chr5750         | Bracer Regina
5259  | chr5695         | Grani
5300  | chr5403         | Shino
5301  | chr0580a        | Guide Senri
5350  | chr5751         | Bracer Xiaoling | btlface0_c5751
5351  | chr0833         | Bracer Lahad
5352  | rob0010         | Ishtanti
5353  | chr5116         | Olympia (Combat Walla)
5400  | chr5401         | Hermès
5450  | chr0712_c01     | Ethan
5451  | chr0427         | Glasgow
5550  | chr0413         | Ronald
5551  | chr0668         | Man Dressed As Construction Worker
5700  | chr0510b        | Terrorist Remnant
5800  | chr0790         | Jaeger Dropout | btlface0_c0790
5801  | chr0790         | Jaeger Dropout | btlface0_c0790
5802  | chr0790         | Jaeger Dropout | btlface0_c0790
5803  | chr0790         | Jaeger Dropout | btlface0_c0790
5850  | chr0426         | Senator Ryan
8000  | chr0500         | Lott | btlface0_c0500
8001  | chr0503         | Daniel | btlface0_c0503
8002  | chr0787         | Gangster | btlface0_c0785
8003  | chr5503a        | Alicia
8004  | chr0710         | Man | btlface0_c0711_c01
8005  | chr0710         | Jonas
8006  | chr5504         | Jessica
8007  | chr0500b        | Neil
8008  | chr5680         | Kwan | btlface0_c5680
8009  | chr5511b        | Caroline | btlface0_c5511b
8010  | chr0512a        | Samson
8011  | chr0511a        | Richie
8012  | chr5500b        | Amy
8013  | chr0745         | Percy
8014  | chr0730         | Station Worker
8015  | chr0521a        | Bruno
8016  | chr0710         | Roderick
8017  | chr5541         | Lyla
8018  | chr5500         | Laura
8019  | chr5541         | Laura's Daughter
8020  | chr0620         | Manager Fazil
8021  | chr5630_c01     | Sinem
8022  | chr0513a        | Marcello
8023  | chr0660_c01a    | Officer Camilo | btlface0_c0660_c01a
8024  | chr0610         | Caravan Chief Osman
8025  | chr0500         | Tourist | btlface0_c0500
8026  | chr5715         | Juliana
8027  | chr0750         | Archaeologist Inigo | btlface0_c0750
8028  | chr0785_c01     | Jeffrey | btlface0_c0785_c01
8029  | chr0501a        | Stephan
8030  | chr0510         | Carson
8031  | chr5521         | Dolly
8032  | chr0711a        | Barton
8033  | chr0514_c01     | Guru
8034  | chr0404         | Lucio
8035  | chr0501         | Tebo | btlface0_c0501
8036  | chr0712a        | Roger | btlface0_c0712a
8037  | chr0786         | Damian | btlface0_c0786
8038  | chr0731         | Security Guard A
8039  | chr0731         | Security Guard
8040  | chr0740         | Harrison
8041  | chr5720_c01     | Female Researcher?
8042  | chr0717a        | Clément
8043  | chr5521         | Daphne
8044  | chr5670         | Store Clerk | btlface0_c0711_c01
8045  | chr5504         | Amalie
8046  | chr0403         | President Daltie
8047  | chr0715_c01     | Editor-in-Chief Cunning
8048  | chr5670         | Manager
8049  | chr0511         | Walton
8050  | chr5710_c01     | Juliana
8051  | chr5711_c01     | Officer Vernier
8052  | chr5711_c02     | Cervi
8053  | vcar072         | Radio-Controlled Land Carrier
8054  | vcar002         | Radio-Controlled Pickup Truck
8055  | chr5503a        | Francisca
8056  | chr0511b        | Secondhand Dealer
8057  | chr5521         | Elderly Woman
8058  | chr5502a        | Gawking Woman
8059  | chr0500b        | Film Crew | btlface0_c0501
8060  | chr0501         | Film Crew | btlface0_c0501
8061  | chr5675         | Manager
8062  | chr0510_c01     | Man on Walk
8063  | chr5671_c01     | Stalker
8064  | chr0722         | Manager Thomson
8065  | chr0785_c02     | Desire Remnant
8066  | chr0785a        | Gangster | btlface0_c0785
8067  | chr0786a        | Gangster | btlface0_c0785
8068  | chr0787         | Gangster | btlface0_c0785
8069  | chr0787a        | Gangster | btlface0_c0785
8070  | chr0786         | Gangster | btlface0_c0785
8071  | chr0787a        | Gangster | btlface0_c0785
8072  | chr0425         | Jed
8073  | chr0513_c01     | Jed's Father
8074  | chr0710_c01     | Deen
8075  | chr0712_c01     | Jet
8076  | chr5551a        | Rui Li
8077  | chr0715_c02     | Brothel Manager
8078  | chr0717a        | Wilmer
8079  | chr5410         | Kisara
8080  | chr0785a        | Young Man | btlface0_c0500
8081  | chr0786a        | Young Man | btlface0_c0500
8082  | chr0503b        | Ned
8083  | chr5680a        | Store Clerk | btlface0_c0711_c01
8084  | chr0712a        | Man | btlface0_c0711_c01
8085  | chr0711_c01     | Man | btlface0_c0711_c01
8086  | chr0780         | Marduk Security Guard | btlface0_c0780
8087  | chr0780_c01     | Marduk Security Guard | btlface0_c0780
8088  | chr0701         | Mengxiao
8089  | chr0848         | Head Chef Yon Tzu
8090  | chr5550         | Female Customer
8091  | chr5501c        | Female Customer
8092  | chr0500a        | Male Customer | btlface0_c0503
8093  | chr0551         | Male Customer | btlface0_c0503
8094  | chr0560         | Angler
8095  | chr0561a        | Angler
8096  | chr0716_c01a    | Judge
8097  | chr0570         | Judge
8098  | chr5560a        | Judge
8099  | chr0716         | Minhou
8100  | chr0710b        | Shipping Worker
8101  | chr0757         | Devancy Eldest Son | btlface0_c0757
8102  | chr0731_c01     | Devancy Middle Son | btlface0_c0731_c01
8103  | chr0665_c01     | Devancy Youngest Son | btlface0_c0665_c01
8104  | vcar022c        | Dragon Chariot
8105  | chr5675a        | Teresa
8106  | chr0711_c01     | Staff Member | btlface0_c0711_c01
8107  | chr0665         | Man Handing Out Balloons
8108  | chr5511b        | Suspicious Woman | btlface0_c5511b
8109  | chr0511b        | Conrad
8110  | chr0710_c01     | Party Planner
8111  | chr0419         | Dunne
8112  | chr0412         | Hubert
8113  | chr0724         | Kenny
8114  | chr0511         | Unsavory Customer
8115  | chr0512a        | Suspicious Man
8116  | chr5504b        | Dorothea
8117  | chr0806         | Boys' Basketball Team Captain
8118  | chr0805_c10     | Boys' Basketball Team Manager
8119  | chr5700_c03     | Girls' Basketball Team Manager
10001 | veh0000         | Pickup Truck
10002 | veh0001         | Upgraded Pickup Truck
10003 | veh5001         | Orbal Motorcycle (Bergard)
10004 | veh0002         | Maxim's Vehicle (Supercar)
10005 | vcar042         | Maxim's Vehicle (Light)
10006 | veh5002         | Hermès' Motorcycle
10007 | etc0200_c00     | Interior Announcement
10008 | etc0200_c00     | Interior Announcement
10009 | etc0200_c00     | Interior Announcement
10010 | etc0200_c00     | Street Screen
10011 | etc0200_c00     | Street Screen
11000 | chr0661_c01     | Sr. Officer Walley
11001 | chr0666         | Boss Goddard
11002 | chr0730         | Driver Roger
11003 | chr5500b        | Maury
11004 | chr0511         | Nathan
11005 | chr0407         | Kyle
11006 | vbike000        | Kyle's Bicycle
11007 | chr5505         | Melissa
11008 | chr5650a        | Sister Belicia
11009 | chr0521         | Hunter Rutts
11010 | etc0003         | Hunting Dog Garland
11011 | chr0561         | Ranji
11012 | chr5560         | Motona
11013 | chr0513_c01     | Gonzales
11014 | chr5513         | Giovanna
11015 | chr5531_c01     | Annie
11016 | chr0541         | Kent
11017 | chr0611         | Jafar
11018 | chr0630         | Lashkar
11019 | chr5522a        | Martha
11020 | chr0667         | Focken
11021 | chr0665         | Hans
11022 | vcar060         | Hans' Truck
11023 | chr0510         | Ben
11024 | chr5503a        | Ran
11025 | chr0716         | Kim
11026 | chr5795_c02     | Kaina
11027 | chr0570_c01     | Mayor Weng
11028 | chr5520b        | Annha
11029 | chr0655_c01     | Vicar Johan
11030 | etc0010         | Nero
11031 | vcar081         | Bus
11032 | vcar000         | Pickup Truck
11033 | chr0115_c01     | Melchior
11050 | chr0532         | Harry
11051 | chr0540         | Mark
11052 | chr5531         | Sheena
11053 | chr0511         | Hawker
11054 | chr0716_c02     | Hawker
11055 | chr5522         | Natalia
11056 | etc0010         | Étoile
11057 | chr5400         | Gin
11058 | chr5795         | Rebecca
11059 | chr0670         | Chris
11060 | chr5510         | Brenda
11061 | chr0500         | Mikkel | btlface0_c0500
11062 | chr5530         | Anya
11063 | chr0521         | Huck
11064 | chr0720_c01     | Huck
11065 | chr5513         | Anya's Mother
11066 | chr0655         | Vicar Nikolai
11067 | chr5650         | Sister Myria
11068 | chr0548         | Curtis
11069 | chr0510_c01     | Roman
11070 | chr5511a        | Zora
11071 | chr0411         | Jess
11072 | chr0501a        | Raymond
11073 | chr5500         | Carol
11074 | chr5671_c01     | Carol
11075 | chr5501a        | Monique
11077 | chr0510a        | Robinson | btlface0_c0510a
11078 | chr5513b        | Margaret
11079 | chr0786_c01     | Eddy
11080 | chr0715         | Neumann
11081 | chr0661         | Sr. Officer Paul
11082 | chr5675         | Melise
11083 | chr5510a        | Clarissa
11084 | chr5540         | Myna
11085 | chr0408         | Giordano
11086 | vcar025         | Giordano's Taxi
11087 | chr0500b        | Clutch
11088 | chr5501b        | Gretta
11089 | chr5502a        | Kopeyka
11090 | chr0834         | Bracer Gray
11091 | chr0520_c01     | Hazel
11092 | chr0712b        | Ilhan
11093 | chr0711         | Manager Carlo
11094 | chr5671         | Isla
11095 | chr5511_c01     | Olivia
11096 | chr0750_c01     | Butler Markley
11097 | chr5720_c01     | Kei
11098 | chr0720         | Administrator Wesley
11099 | chr5513_c01     | Manager Deborah
11100 | chr0551         | Li-Lian | btlface0_c0551
11101 | chr5510b        | Marguerite
11102 | chr5795_c01     | Sylvie
11103 | chr5503b        | Beth
11104 | vcar064         | Shino's Food Truck
11105 | vcar080         | Orbal Long Distance Bus
11106 | vcar021         | Sedan 1
11107 | vcar040         | Light 1
11108 | vcar081         | Orbal City Bus
11109 | vcar041         | Light 2
11110 | vcar060         | Light Truck
11111 | vcar022         | Sedan 3
11112 | vcar072         | Jeep
11113 | vcar042         | Light 1
11114 | vcar050         | Limousine 1
11115 | vcar061         | Small Truck
11116 | chr0405         | Jacobin
11117 | chr0831         | Bracer Ewan
11118 | chr5670         | Milty
11120 | chr0570a        | Secondhand Dealer Yimai
11121 | chr0666         | Morgan
11122 | chr5600a        | Maja
11123 | chr5503a        | Lindsey
11124 | chr5711a        | Mistress
11125 | chr0715c        | Barker Tracey
11126 | chr0513_c01     | Beckson
11127 | chr5405         | Gabriella
11128 | chr0541         | Yota
11129 | chr5715_c01a    | Candy
11130 | chr0400         | Joe Browning
11131 | chr5520         | Magisa
11132 | chr5710a        | Paprica
11133 | chr5404         | Doctor Isabelle
11134 | chr5705         | Maron
11135 | chr0715_c02     | Well-Dressed Man
11136 | chr0790         | Jaeger | btlface0_c0790
11137 | chr0727         | Dolor Family 1 | btlface0_c0727
11138 | chr0727_c01     | Dolor Family 2 | btlface0_c0727_c01
11139 | chr0791         | Jaeger Dropout 1 Alt Color (Zahat's Scorpion) 1 | btlface0_c0791
11140 | chr0791_c01     | Jaeger Dropout 1 Alt Color (Zahat's Scorpion) 2 | btlface0_c0791_c01
11150 | chr0745         | Shakar
11151 | chr0701         | Kaohan
11152 | chr5551         | Kunming
11153 | chr5561         | Owner Mau
11154 | chr5670         | Receptionist Rianna
11155 | chr5675a        | Editor-in-Chief Sullivan
11156 | chr0711b        | Reporter Chen
11157 | chr0715a        | Reporter Langdon
11158 | chr5505         | Giselle
11159 | chr5501c        | San
11160 | chr5504b        | Marilyn
11161 | chr0501         | Peter | btlface0_c0501
11162 | chr0501a        | Wendell
11163 | chr0502b        | Sloth
11164 | chr5500a        | Abby
11165 | chr5660_c01     | Officer Karen
11166 | chr5660         | Officer Nancy
11167 | chr0712         | Porco | btlface0_c0712
11168 | chr5695a        | Nami
11169 | chr0510b        | Owner Charles
11170 | etc0011         | Kageyoshi
11171 | chr5706         | Lucia
11172 | chr5610         | Khawla
11173 | chr0640         | Lolo
11174 | chr5676         | Manager Giulia
11175 | chr0716a        | Transcent
11176 | chr5680_c01     | Receptionist Polecca
11177 | chr0717a        | Guide Hendrick
11178 | chr0722         | Tokey
11179 | chr5532         | Cathy
11180 | vcar060         | Small Truck
11181 | vcar072         | Jeep
11182 | vcar040         | Light 1
11183 | vcar080         | Orbal Bus
11184 | vcar030         | Sports Car 1
11185 | vcar020         | Sedan 1
11186 | vcar081         | Orbal City Bus
11187 | vcar051         | Limousine 2
11188 | vcar021         | Sedan 1
11189 | vcar062         | Mid-Size Truck
11190 | chr0787         | Delinquent | btlface0_c0787
11191 | chr5600a        | Aslan's Fan
11192 | chr5504         | Aslan's Fan
11193 | chr0710_c01     | Meet-and-Greet Staff Member
11194 | chr0710a        | Meet-and-Greet Staff Member
11195 | chr0717c        | Salif
11196 | chr0717a        | Politician
11197 | chr0715_c02     | Politician
11198 | chr5510a        | Cathy's Mother
11200 | chr5730         | Cocona
11201 | chr0710         | Cronin
11202 | chr0666         | Bradley
11203 | chr5502         | Junifer
11204 | vcar020c        | Junifer's Car
11205 | chr0503b        | Graham
11206 | vcar031         | Graham's Car
11207 | chr5790_c01     | Seyla
11208 | chr5711         | Natalie | btlface0_c5711
11209 | chr5791_c01     | Lulua
11210 | chr0657         | Vicar Mansur
11211 | chr5652         | Priest Kyra
11212 | chr5501         | Selma
11213 | chr0501b        | Lodi
11214 | vbike002        | Lodi's Bicycle
11215 | chr0513         | Gordon
11216 | chr5504a        | Primo
11217 | chr0530         | Rooney
11218 | chr5640a        | Irle
11219 | chr5406         | Shannon
11220 | chr5407         | Makino
11221 | chr0611         | Ponty
11222 | vcar060         | Ponty's Car
11223 | chr0502         | Ticket Clerk
11224 | vcar020         | Sedan 1
11225 | vcar021         | Sedan 2
11226 | vcar064         | Sedan 3
11227 | chr0660         | Police Officer
11228 | chr0513a        | Reporter | btlface0_c0711_c01
11229 | chr0711a        | Photographer
11230 | chr0716_c01     | Gossip Reporter
11231 | chr0780         | Marduk Security Guard | btlface0_c0780
11232 | chr0780_c01     | Marduk Security Guard | btlface0_c0780
11233 | chr0780         | Marduk Security Guard | btlface0_c0780
11234 | chr5790         | Seyla
11235 | chr5791         | Lulua
11236 | chr5402_c01     | Selma
11237 | g00hou02        | Roasted Chestnut Stand
11238 | chr5611a        | Irle's Mother
11250 | chr0716         | Oslo
11251 | chr0502         | Simon
11252 | chr0549         | Kirk
11253 | chr5630         | Frawla
11254 | chr5531         | Sue
11255 | chr5503         | Mariquita
11256 | chr0660_c01     | Officer Young
11257 | chr0848         | Kanza
11258 | chr0661_c01     | Sr. Officer Colordo
11259 | chr5660_c01     | Receptionist Kiara
11260 | chr0720_c02     | Rumsfeld
11261 | chr5676a        | Cremona
11262 | chr0720         | Senator Polanski
11263 | chr5675_c01     | President Jennifer
11264 | chr5532a        | Berzelia
11265 | chr0725         | Security Guard Simmons | btlface0_c0725
11266 | chr5711_c01     | Manager Mohra
11267 | chr0531         | Allen
11268 | vcar040         | Light 1
11269 | vcar070         | Jeep
11270 | vcar021         | Sedan 2
11271 | vcar022         | Sedan 3
11272 | vcar081         | Orbal City Bus
11273 | vcar051         | Limousine 2
11274 | vcar030         | Sports Car 1
11275 | vcar020         | Sedan 1
11276 | vcar040         | Light 1
11277 | vcar080         | Orbal Bus
11278 | vcar041         | Light 2
11279 | vcar021         | Sedan 2
11280 | vcar020c        | Sedan 3
11281 | vcar031         | Sports Car 2
11282 | vcar081         | Orbal City Bus
11283 | vcar050         | Limousine 1
11284 | vcar040         | Light 1
11285 | vcar030         | Sports Car 1
11286 | vcar020         | Sedan 1
11287 | vcar021         | Sedan 2
11288 | chr0710         | Elite-Looking Young Adult
11300 | chr0401         | Secretary Culleo
11301 | chr5513b        | Francois
11302 | etc0012         | Madelaine
11303 | chr0520a        | Hopkins
11304 | chr5530a        | Eve
11305 | etc0001         | Apollo
11306 | chr0660         | Officer Sergi
11307 | chr5402         | Ms. Sonia
11308 | chr0703         | Holland
11309 | chr5502b        | Lisa
11310 | chr0652         | Priest Alfred
11311 | chr0655_c01     | Priest Watson
11312 | chr0658         | Bishop Orwell
11313 | chr5655         | Sister Orze
11314 | chr5650_c01     | Sister Philia
11315 | chr5513a        | Nadiya
11316 | chr5541         | Amia
11317 | vcar050         | Limousine 1
11318 | vcar020         | Sedan 1
11319 | vcar022         | Sedan 3
11320 | vcar051         | Limousine 2
11321 | chr5402_c02     | Ms. Sonia
11325 | rob0002         | Assault Frame
11326 | rob0002         | Assault Frame
11327 | vcar090         | Republic Tank
11328 | vcar090         | Republic Tank
11329 | chr0755         | Soldier | btlface0_c0755
11330 | chr0755_c01     | Soldier | btlface0_c0755
11331 | chr0755         | Soldier | btlface0_c0755
11332 | chr0755_c01     | Soldier | btlface0_c0755
11350 | chr5511_c02     | Tricia
11351 | chr0711b        | Hanjiao
11352 | chr5705a        | Suiran
11353 | chr0700         | Vittorino
11354 | chr5561a        | Xue
11355 | chr0590         | Yao
11356 | chr0570a        | Old Man Wang
11357 | chr0665         | Deliveryman Anthony
11358 | chr0716_c01a    | Touan
11359 | chr0502a        | Downey
11360 | chr0500a        | Hyades
11361 | chr5680b        | Guide Yilin
11362 | chr0750_c01     | Detective Dart
11363 | chr0704         | Owner Kuwano
11364 | chr5695         | Sicily
11365 | chr0501b        | Darren
11366 | chr5504a        | Katyusha
11367 | chr5670b        | Receptionist Humin
11368 | chr0717b        | Hongfa
11369 | chr5551a        | Aimin
11370 | chr0670a        | Wulai
11371 | chr5513a        | Josephine
11372 | chr0510c        | Liu-Ho
11373 | chr0716_c02     | Hanson
11374 | chr5676b        | Kumei
11375 | chr0717b        | Laihong
11376 | chr0715a        | Customs Officer Rafale
11377 | chr0600a        | Faysal
11378 | chr0756_c01     | MCPO Abrams
11379 | chr0756         | Navy Ensign Glasley
11380 | chr0668         | Rosaro
11381 | chr0570a        | La-Kau
11382 | chr5550         | Miren
11383 | chr5560a        | Lanfa
11384 | chr5590         | Linlin
11385 | chr0560a        | Jiafei
11386 | chr0745         | Heydrich
11387 | chr0551a        | Qintao
11388 | chr5580         | Fuling
11389 | chr0580         | Tau
11390 | chr5570         | Pao
11391 | chr0795         | Haifeng | btlface0_c0795
11392 | chr0570         | Ouran
11393 | chr0561a        | Ferryman Shou
11394 | chr0715         | Manager Hairen
11395 | chr5782         | Lince
11396 | chr0850         | Kan
11397 | chr5671         | Receptionist Fuuka
11398 | chr5730_c01     | Amalia
11399 | chr0704         | Wuhan
11400 | chr5561_c01     | Haimin
11401 | chr0406         | Filenkov
11402 | chr0656         | Vicar Hou-Seng
11403 | chr5651         | Priest Sei-Lan
11404 | chr0716a        | Mouton
11405 | chr0724_c01     | Poisson
11406 | chr0561_c01     | Shitao
11407 | chr0513b        | Gomez
11408 | vcar029         | New Vehicle
11409 | vcar020b        | Old Vehicle
11410 | chr5710         | Hostess | btlface0_c5710
11411 | chr5711         | Hostess | btlface0_c5710
11412 | vcar082         | Langport-Colored City Bus
11413 | vcar022         | Sedan 3
11414 | vcar022         | Sedan 3
11415 | vcar070         | Jeep
11416 | vcar041         | Light 2
11417 | vcar082         | Langport-Colored City Bus
11418 | vcar051         | Limousine 2
11419 | vcar051         | Limousine 2
11420 | vcar022         | Sedan 3
11421 | vcar051         | Limousine 2
11422 | vcar082         | Langport-Colored City Bus
11423 | vcar022         | Sedan 3
11424 | vcar022         | Sedan 3
11425 | vcar070         | Jeep
11426 | vcar040         | Light 1
11427 | vcar082         | Langport-Colored City Bus
11428 | vcar051         | Limousine 2
11429 | vcar051         | Limousine 2
11430 | chr0319_c00     | Houseman
11431 | chr0711a        | Receptionist Calvin
11432 | chr0406_c00     | Filenkov
11433 | chr0835         | Bracer Hornet
11450 | chr0601a        | Darvish
11451 | chr5660a        | Officer Vernier
11452 | chr0600         | Ihsan
11453 | chr0721a        | Butler Hardy
11454 | chr5511         | Orsini
11455 | chr5531a        | Elene
11456 | chr0705_c01     | Muhsin
11457 | chr0715c        | Barker Zaltz
11458 | chr0512b        | Bobly
11459 | chr0660_c01     | Officer Kelvin
11460 | chr0716_c01     | Clemens
11461 | chr0503a        | Nino
11462 | chr5500a        | Cervi
11463 | chr5601a        | Philica
11464 | chr0811         | Sparks
11465 | chr5501_c01     | Nate's Girlfriend
11466 | chr0610a        | Shadīn
11467 | chr5611a        | Majelah
11468 | chr0669a        | Daskal
11469 | chr5680_c01a    | Receptionist Salma
11470 | chr5501a        | Shayla
11471 | chr5502b        | Laure
11472 | chr0720a        | Manager David
11473 | chr0116_c00     | Harold
11474 | chr0724_c01     | Dealer
11475 | chr0724         | Lester
11476 | chr0610_c01     | Mourad
11477 | chr5711b        | Amala
11478 | chr5682a        | Rayhana
11479 | chr5671         | Receptionist Clammy
11480 | chr0711a        | Isaac
11481 | chr5676a        | Chief Manager Dianne
11482 | chr5670c        | Receptionist Maawa
11483 | chr0725_c02     | Bolhos | btlface0_c0725_c02
11484 | chr0724         | Peterson
11485 | chr5675_c01     | Executive Director Suzanne
11486 | chr0717c        | Dahar
11487 | chr5532b        | Kaliyah
11488 | chr0611a        | Patriarch Bashar
11489 | chr0631         | Mundt
11490 | chr5740         | Berthier
11491 | chr0611         | Rasham
11492 | chr5640         | Imani
11493 | chr0540a        | Clim
11494 | chr0511b        | Ulrich
11495 | chr5510         | Jasmine
11496 | chr5610a        | Shahana
11497 | chr0620         | Abdullah
11498 | chr0620_c01     | Zaham
11499 | chr5630a        | Pichcha
11500 | chr0702         | Jaret
11501 | chr5620         | Balsheh
11502 | chr5601         | Monir
11503 | chr0601a        | Khosrow
11504 | chr0610         | Rahmani
11505 | chr0657         | Vicar Mukhtar
11506 | chr5652         | Priest Jamileh
11507 | chr5600         | Fet
11508 | chr5671b        | Receptionist Jibril
11509 | chr0601         | Mahdi
11510 | vcar030         | Ihsan's Sports Car
11511 | etc0020         | Rahmani's Camel
11512 | vcar072         | Mahdi's Jeep
11513 | vcar021         | Sedan 2
11514 | vcar022         | Sedan 3
11515 | vcar040         | Light 1
11516 | vcar051         | Limousine 2
11517 | vcar025b        | Taxi
11518 | vcar026         | Patrol Car
11519 | vcar072         | Jeep
11520 | vcar063         | Display Vehicle
11521 | vcar021         | Sedan 2
11522 | vcar041         | Light 2
11523 | vcar025b        | Taxi
11524 | vcar020         | Sedan 1
11525 | vcar022         | Sedan 3
11526 | vcar050         | Limousine 1
11527 | vcar030         | Sports Car 1
11528 | vplane000       | Airship
11550 | chr5522a        | Heidi
11551 | chr0666         | Boss Grover
11552 | chr0669         | Deliveryman Jason
11553 | chr0710a        | Barnaby
11554 | chr0402         | Researcher Ray
11555 | chr0513_c01     | Boss Gowland
11556 | chr0670         | Voyager
11557 | chr0520         | Stinson
11558 | chr0503         | Gillies | btlface0_c0503
11559 | chr0668_c01     | Deliveryman Marco
11560 | chr0512a        | Lawrence
11561 | chr0712         | Receptionist Alex | btlface0_c0712
11562 | chr0655         | Vicar Flictam
11563 | chr5650         | Sister Paula
11564 | chr0671         | Chief Manager Eugen
11565 | chr5795         | Engineer Riel
11566 | chr0541a        | Nat
11567 | chr5670a        | Pier
11568 | chr0715a        | Banker Oliver
11569 | chr5530a        | Millia
11570 | chr0661_c01     | Sr. Officer Minster
11571 | chr5680a        | Receptionist Echo
11572 | chr0735         | Researcher Selion
11573 | chr0730         | Station Worker Carrick
11574 | chr0501         | Willy | btlface0_c0501
11575 | chr5680         | Guide Finse | btlface0_c5680
11576 | chr0716_c02     | Chamberlain
11577 | chr5675a        | Petra
11578 | chr0530a        | Dick
11579 | chr5676         | Chief Valeria
11580 | chr0720         | President Rooster
11581 | chr0724_c02     | Receptionist Freyd
11582 | chr0711b        | Lanmei
11583 | chr0717         | Manager Radad
11584 | chr0665         | Engineer Carter
11585 | chr5513b        | Anna
11586 | chr5720         | Pharmacist Luce
11587 | chr0500a        | De Niro
11588 | chr0740         | Coolidge
11589 | chr0660         | Police Officer
11590 | chr0716_c01     | Office Worker Oluger
11591 | chr5503b        | Meryl
11592 | chr0716a        | Librarian Venter
11593 | chr5725         | Professor Aurelie
11594 | chr0740_c01     | Associate Professor Tain
11595 | chr5720_c01a    | Researcher Athena
11596 | chr0410         | Researcher Lionel
11597 | chr0735_c01     | Moritz
11598 | chr0735a        | Gimbat
11599 | chr0502a        | Lucas
11600 | chr5500b        | Liz
11601 | chr5720         | Researcher Cindy
11602 | chr0735         | Researcher Aster
11603 | chr0735_c01a    | Researcher Nadir
11604 | chr5521         | Elderly Woman
11605 | vcar064         | Roaming Café
11606 | vcar070         | Guild Jeep
11607 | vcar040         | Generic Orbal Car
11608 | chr5005_c06     | Risette
11609 | vcar061         | Delivery Truck
11610 | mon0042_c00     | Basel Monster 1
11611 | mon0041_c00     | Basel Monster 2
11612 | mon0048         | Basel Monster 3
11613 | mon0012_c01     | Basel Monster 4
11614 | mon0049         | Basel Monster 5
11650 | chr5570b        | Receptionist Drusilla
11651 | chr0832         | Bracer Kazan | btlface0_c0832
11652 | chr0656         | Vicar Murase
11653 | chr5651         | Priest Izumi
11654 | chr0845_c01     | Tsuruji
11655 | chr5570a        | Omoro
11656 | chr0711         | Tourism Bureau Worker
11657 | chr0847         | Kukai
11658 | chr0730         | Station Worker Cordeaux
11659 | chr0550         | Rito
11660 | chr5735         | Ludesia
11661 | chr0561a        | Gengfu
11662 | chr0661         | Sr. Officer Lambert
11663 | chr0846         | Walker
11664 | chr5761         | Torte
11665 | chr0745         | Rodrigo
11666 | chr5730         | Samantha
11667 | chr5590a        | Ino
11668 | chr5550a        | Nanai
11669 | chr0561_c01     | Shigi
11670 | chr5561_c01     | Fumi
11671 | chr0848_c01     | Manager Mamiya
11672 | chr0849         | Head Chef Hakurou
11673 | chr5770         | Hostess Mikasa
11674 | chr0560a        | Tousen
11675 | chr5560         | Sumi
11676 | chr0501a        | Novic
11677 | chr5531         | Lotta
11678 | chr0550a        | Taihong | btlface0_c0550a
11679 | chr5118_c01     | Woman in a Kimono
11750 | chr5513b        | Director Flona
11751 | chr0532a        | Jean
11752 | chr5531_c01     | Sophie
11753 | chr0541         | Todd
11754 | chr5540b        | Rinka
11755 | chr5655         | Sister Beatrice
11756 | chr0730         | Attendant Wood
11757 | chr0660_c01     | Officer Endeavor
11758 | chr0521a        | Cambridge
11759 | chr5510b        | Constance
11760 | chr0510a        | Sylvan | btlface0_c0510a
11761 | chr5513a        | Goethe
11762 | chr0502b        | Sutton
11763 | chr5501b        | Morie
11764 | chr0500         | Ricardo | btlface0_c0500
11765 | chr5532a        | Tietta
11766 | chr5541         | Paulina
11767 | chr0717a        | Gustav
11768 | chr5511_c02     | Sabine
11769 | chr5511b        | Germay | btlface0_c5511b
11770 | chr5513_c01     | Diane
11771 | chr0660         | Officer Pascal
11772 | chr5660_c01     | Officer Katarina
11773 | chr0712a        | Norbert | btlface0_c0712a
11774 | chr5706a        | Nelly
11775 | chr5735_c01     | Emma
11776 | chr0669         | Boss Philibert
11777 | chr0661         | Sr. Officer Irons
11778 | chr0514         | Baron Baudoin
11779 | chr0531a        | Manuel
11780 | chr5531a        | Charlene
11781 | chr0716         | Horton
11782 | chr0710         | Secretary Inno
11783 | chr5521         | Adelina
11784 | chr0530a        | Pino
11785 | chr0652_c01     | Priest Enel
11786 | chr5510         | Rosanna
11787 | chr0540a        | Harriet
11788 | chr5671a        | Irene
11789 | chr5675_c01     | Chief Manager Parler
11790 | chr5735         | Mathilde
11791 | chr0667         | Gaston
11792 | chr0731         | Security Guard Belfast
11793 | chr5511a        | Lovisa
11794 | chr5540a        | Rennes
11795 | chr0721         | Manager Delanoe
11796 | chr5682         | Receptionist Gelee
11797 | chr0717_c01     | Banker Dupont
11798 | chr0510_c01     | Doyen
11799 | chr5510a        | Vanessa
11800 | chr5660         | Officer Crema
11801 | chr0501b        | Cepages
11802 | etc0000         | Flick
11803 | chr5504b        | Amelie
11804 | chr5520a        | Desiree
11805 | chr5513         | Bernice
11806 | chr5522         | Timo
11807 | chr0700         | Andy
11808 | chr0750         | Ungern | btlface0_c0750
11809 | chr5505         | Jolanda
11810 | chr0504         | Westache
11811 | chr5730_c01     | Fia
11812 | chr5541b        | Teto
11813 | chr5676a        | Receptionist Marion
11814 | chr0724_c02     | Cesar
11815 | chr0513b        | Keegan
11816 | chr0512b        | Pershing
11817 | chr5504         | Receptionist Luka
11818 | chr0837         | Bracer Kirchheim
11819 | chr0710a        | Reporter Ferdinand
11820 | vcar026         | Officer Pascal's Patrol Car
11821 | veh0003         | Lucio's Vehicle (Supercar)
11850 | chr5501_c01     | Nate's Girlfriend
11851 | chr5610a        | Latifa
11852 | chr5408         | Gigi
11853 | chr5550         | Eastern Young Woman
11854 | chr0550a        | Eastern Young Adult | btlface0_c0550a
11900 | chr5510a        | Pamela
11901 | chr0805_c07     | Elijah
11902 | chr5414         | Ms. Matilda
11903 | chr5700_c04     | Thelma
11904 | chr5700_c05     | Sigourney
11905 | chr0805_c04     | Edmond
11906 | chr5700_c06     | Ariya
11907 | chr0805_c05     | Woody
11908 | chr0805_c06     | Donatello
11909 | chr5700_c07     | Frederica
11910 | chr5413         | Ms. Leifa
11911 | chr0423         | Mr. Raffaello
11912 | chr0503         | Roscoe | btlface0_c0503
11913 | chr0805_c08     | Boys' Basketball Team Member
11914 | chr0806         | Boys' Basketball Team Member
11915 | chr0500a        | Male Customer | btlface0_c0503
11916 | chr0503         | Male Customer | btlface0_c0503
11917 | chr5513_c01     | Lyle's Mother
11918 | chr0530a        | Lyle's Brother
11919 | chr5541         | Lyle's Sister
11920 | chr0710         | Edward
11921 | chr0513         | Dine-And-Dasher
11922 | chr0807_c01     | ◆NPC Student By Stands Outside School
11923 | chr0807_c02     | ◆NPC Student By Stands Outside School
11924 | chr5701_c01     | ◆NPC Student By Stands Outside School
11925 | chr5700_c09     | Female Student
11926 | chr0805_c03     | Male Student
11927 | chr5700_c10     | Female Student
11928 | chr5703         | Female Student
11929 | chr0805_c03     | Male Student
11930 | chr5700_c02     | Female Student
11931 | chr5700_c03     | Female Student
11932 | chr0805_c03     | Male Student
11933 | chr5701         | ◆Bazaar NPC Student
11934 | chr0807_c01     | ◆Bazaar NPC Student
11935 | chr0807_c02     | ◆Bazaar NPC Student
11936 | chr0806         | ◆Courtyard Stall NPC Student
11937 | chr5701_c01     | ◆Courtyard Stall NPC Student
11938 | chr0806_c01     | ◆Courtyard Stall NPC Student
11939 | chr5701         | ◆Courtyard Stall NPC Student
11940 | chr5540b        | Lost Child
11941 | chr5510         | Visitor
11942 | chr5531_c01     | Visitor's Daughter
12000 | chr5411         | Meixin
12001 | chr5580         | Tiantian
12002 | chr0550         | Yongfu
12003 | chr5560         | Tanwa
12004 | chr0560         | Beifan
12005 | chr0422         | Master Tailong
12006 | chr5412         | Binsui
12007 | chr0860         | Eichou
12008 | chr0561a        | Guide Kesen
12009 | chr5710_c02     | Lairei
12010 | chr5570         | Granny Honghua
12011 | chr5590         | Yunyun
12012 | chr0532a        | Sooyoung
12013 | chr0580         | Xingqi
12014 | chr0550a        | Koki | btlface0_c0550a
12015 | chr0551_c01     | Ryusei
12016 | chr5551a        | Mion
12017 | chr0511b        | Mirabo
12018 | vcar082         | Langport-Colored City Bus
12019 | vcar022         | Sedan 3
12020 | vcar070         | Jeep
12021 | vcar040         | Light 1
12022 | chr0501b        | Nick
12100 | chr0724_c01     | Mr. Ward
12101 | chr5705a        | Romy
12102 | chr0512a        | Elton
12103 | chr5611a        | Jumaana
12104 | chr0520_c01     | Moses
12105 | chr5522         | Belinda
12106 | chr0520a        | Grandpa Ahab
12107 | chr0521         | Old Man Romain
12108 | chr5513         | Genista
12109 | chr5541b        | Peggy
12110 | chr0513_c01     | Trout
12111 | chr5600a        | Urfa
12112 | chr0512b        | Yazan
12113 | chr0560a        | Laochao
12114 | chr0500a        | Leonardo
12115 | chr5511_c01     | Barbara
12116 | chr5695         | Sabrina
12117 | chr0511         | Sean
12118 | chr0531a        | Kane
12119 | chr5530a        | Chell
12120 | chr0715b        | Malcolm
12121 | chr5675_c01     | Carina
12122 | chr5503b        | Tabatha
12123 | etc0015         | Tiger
12124 | chr5670a        | Female Reporter
12125 | etc0012         | Kitten
12126 | vcar080         | Orbal Bus
12127 | vcar022         | Sedan 3
12128 | vcar070         | Jeep
12129 | vcar040         | Light 1
12130 | vcar080         | Orbal Bus
12131 | vcar022         | Sedan 3
12132 | vcar070         | Jeep
12133 | vcar040         | Light 1
12200 | chr5680         | Receptionist Esta | btlface0_c5680
12201 | chr0717         | Manager Norris
12202 | chr0500a        | Carter
12203 | chr0703         | Mirco
12204 | chr5706         | Citrus
12205 | chr0513         | Butcher
12206 | chr5520         | Angela
12207 | chr0665         | Ivan
12208 | chr5513b        | Jane
12209 | chr0655         | Vicar Rodion
12210 | chr0652         | Priest Krenz
12211 | chr5650         | Sister Olga
12212 | chr5531a        | Mina
12213 | chr0541a        | Tony
12214 | chr5695         | Josephina
12215 | chr0669         | Sailor Ramsay
12216 | chr0668_c01     | Deliveryman Bosch
12217 | chr0661         | Officer Radovan
12218 | chr0715b        | Mr. Steve
12219 | chr0725_c01     | Robson | btlface0_c0725_c01
12220 | chr5715         | Tiffany
12221 | chr0716         | Lux
12222 | chr0731         | Security Guard Garry
12223 | chr0715c        | Barker Gallico
12224 | chr0785         | Romeo | btlface0_c0785
12225 | chr5710         | Patricia | btlface0_c5710
12226 | chr0520_c01     | Will
12227 | chr0510c        | Mina's Father
12228 | chr5510b        | Tony's Mother
12229 | vcar041         | Light 2
12230 | vcar022         | Sedan 3
12231 | vcar072         | Jeep
12232 | vcar040         | Light 1
12300 | chr0705_c01     | Mr. Ramon
12301 | chr5706         | Brittney
12302 | chr0511a        | Gunnar
12303 | chr5501a        | Pauline
12304 | chr0530a        | Martin
12305 | chr5680a        | Receptionist Elsie
12306 | chr0730         | Security Guard Hugo
12307 | chr0668_c01     | Staff Member Archie
12308 | chr5402_c03     | Betty
12309 | chr0420         | Ralph
12310 | chr0421_c00     | Aslan
12311 | chr0665         | Technician Rodney
12312 | chr5800         | Leslie
12313 | chr5801         | Linmei
12314 | chr0513c        | Carlos
12315 | chr0501a        | Jake
12316 | chr5502b        | Doris
12317 | chr0520_c01     | Ganho
12318 | chr0540a        | Yoki
12319 | chr0512a        | Belmont
12320 | chr5530         | Stephanie
12321 | vcar040         | Light 1
12322 | vcar070         | Jeep
12323 | vcar021         | Sedan 2
12324 | vcar022         | Sedan 3
12325 | vcar081         | Orbal City Bus
12326 | vcar051         | Limousine 2
12327 | vcar030         | Sports Car 1
12328 | vcar020         | Sedan 1
12400 | chr0711_c01     | Guide Ewing | btlface0_c0711_c01
12401 | chr0540         | York
12402 | chr0500b        | Film Crew | btlface0_c0501
12403 | chr0703         | Head Chef Rutherford
12404 | chr5703         | Lydia
12405 | chr5705a        | Janice
12406 | chr0521         | Morley
12407 | chr0669a        | Engineer
12500 | vcar022         | Sedan 3
12501 | vcar050         | Limousine 1
12502 | vcar041         | Light 2
12503 | vcar020         | Sedan 1
12504 | vcar030         | Sports Car 1
12505 | vcar040         | Light 1
12506 | vcar041         | Light 2
12507 | vcar030         | Sports Car 1
12508 | vcar020         | Sedan 1
12509 | vcar041         | Light 2
12510 | vcar022         | Sedan 3
12511 | vcar051         | Limousine 2
12512 | vcar040         | Light 1
12513 | vcar022         | Sedan 3
12514 | vcar050         | Limousine 1
12515 | vcar051         | Limousine 2
12516 | vcar025b        | Taxi
12517 | vcar020         | Sedan 1
12518 | vcar030         | Sports Car 1
12519 | vcar040         | Light 1
12600 | chr0717c        | Manager Iskhak
12601 | chr5680         | Guide Mindy | btlface0_c5680
12602 | chr0710         | Carson
12603 | chr5513a        | Moira
12604 | chr0540         | Bart
12605 | chr5675a        | Dahlia
12606 | chr0600a        | Najib
12607 | chr0424         | Jimmy
12608 | chr0715         | Manager Bernhardt
12609 | chr5511_c02     | Amanda
12610 | chr5504         | Bridget
12611 | chr5670b        | Receptionist Mingyu
12612 | chr5409         | Lila
12613 | chr0712a        | Receptionist Edwin | btlface0_c0712a
12614 | chr5502         | Hannah
12615 | chr0731         | Security Guard Tyson
12616 | chr5680_c01     | Guide Cyndi
12617 | chr0500a        | Dennis
12618 | chr5531_c01     | Julietta
12619 | chr0710a        | Kiefer
12620 | chr5735_c01     | Melanie
12621 | chr0499         | Mishy
12622 | chr5503b        | Rosie
12623 | chr5680_c01a    | Guide Farrah
12624 | chr5500         | Jimmy's Colleague
12625 | chr9992         | Tourist (A)
12626 | chr9992         | Tourist (B)
12627 | chr9992         | Tourist (C)
12628 | vcar050         | Limousine 1
12629 | vcar022         | Sedan 3
12630 | vcar030         | Sports Car 1
12631 | vcar040         | Light 1
12632 | vcar020         | Sedan 1
12633 | chr0711_c01     | Reporter | btlface0_c0711_c01
12634 | chr0670a        | Verne Store Technician
12635 | chr5513         | Julietta's Mother
12636 | chr9992         | Tourist (C)
12637 | chr5671a        | Manager
12700 | chr5540b        | Girl
12701 | chr0540a        | Boy
12702 | chr0717_c02     | Man in Suit
12703 | chr0521a        | Old Man
12704 | chr0785         | Crowd | btlface0_c0785
12705 | chr0786         | Crowd | btlface0_c0785
12706 | chr0787         | Crowd | btlface0_c0785
12707 | chr0790         | Crowd | btlface0_c0785
12708 | chr5715         | Crowd | btlface0_c0785
12709 | chr5611         | Crowd | btlface0_c0785
12710 | chr0790_c01     | Jaeger | btlface0_c0790
12711 | chr0790_c01     | Jaeger | btlface0_c0790
12720 | chr0514_c01     | Sir Vinson
12721 | chr0711_c01     | Store Clerk | btlface0_c0711_c01
12722 | chr9992         | Passenger
12724 | chr5541a        | Myna's Friend
12740 | chr5504b        | Catherine
12741 | etc0011         | Black Cat
12742 | etc0013         | White Cat
12743 | chr0725_c01     | Hotel Employee | btlface0_c0725_c01
12744 | chr5671         | Naomi
12745 | chr0710b        | Secretary
15500 | chr0500         | Young Man 1 (City) | btlface0_c0500
15501 | chr0500a        | Young Man 1 (City; Alt Color Version 1)
15502 | chr0500b        | Young Man 1 (City; Alt Color Version 2)
15503 | chr0501         | Young Man 2 (City) | btlface0_c0501
15504 | chr0501a        | Young Man 2 (City; Alt Color Version 1)
15505 | chr0501b        | Young Man 2 (City; Alt Color Version 2)
15506 | chr0502         | Young Man 3 (City)
15507 | chr0502a        | Young Man 3 (City; Alt Color Version 1)
15508 | chr0502b        | Young Man 3 (City; Alt Color Version 2)
15509 | chr0503         | Young Man 4 (Rural) | btlface0_c0503
15510 | chr0503a        | Young Man 4 (Rural; Alt Color Version 1)
15511 | chr0503b        | Young Man 4 (Rural; Alt Color Version 2)
15512 | chr0504         | Young Man 5 (Aristocratic)
15513 | chr0505         | Young Man 6 (Aristocratic)
15514 | chr0510         | Middle-Aged Man 1 (City)
15515 | chr0510_c01     | Middle-Aged Man 1 (City; Fat Version)
15516 | chr0510a        | Middle-Aged Man 1 (City; Alt Color Version 1) | btlface0_c0510a
15517 | chr0510b        | Middle-Aged Man 1 (City; Alt Color Version 2)
15518 | chr0510c        | Middle-Aged Man 1 (City; Skin/Hair Color Changed to Eastern Version)
15519 | chr0511         | Middle-Aged Man 2 (City)
15520 | chr0511a        | Middle-Aged Man 2 (City; Alt Color Version 1)
15521 | chr0511b        | Middle-Aged Man 2 (City; Alt Color Version 2)
15522 | chr0512         | Middle-Aged Man 3 (City)
15523 | chr0512a        | Middle-Aged Man 3 (City; Alt Color Version 1)
15524 | chr0512b        | Middle-Aged Man 3 (City; Alt Color Version 2)
15525 | chr0513         | Middle-Aged Man 4 (Rural)
15526 | chr0513_c01     | Middle-Aged Man 4 (Rural; Fat Version)
15527 | chr0513a        | Middle-Aged Man 4 (Rural; Alt Color Version 1)
15528 | chr0513b        | Middle-Aged Man 4 (Rural; Alt Color Version 2)
15529 | chr0513c        | Middle-Aged Man 4 (Rural; Skin/Hair Color Changed to Eastern Version)
15530 | chr0514         | Middle-Aged Man 5 (Aristocratic)
15531 | chr0520         | Old Man 1 (City)
15532 | chr0520_c01     | Old Man 1 (City; No Hat)
15533 | chr0520a        | Old Man 1 (City; Alt Color Version 1)
15534 | chr0521         | Old Man 2 (Rural)
15535 | chr0521a        | Old Man 2 (Rural; Alt Color Version 1)
15536 | chr0530         | Young Boy 1 (City)
15537 | chr0530a        | Young Boy 1 (City; Alt Color Version 1)
15538 | chr0531         | Young Boy 2 (City)
15539 | chr0531a        | Young Boy 2 (City; Alt Color Version 1)
15540 | chr0532         | Young Boy 3 (City)
15541 | chr0532a        | Young Boy 3 (City; Alt Color Version 1)
15542 | chr0540         | Little Boy 1 (City)
15543 | chr0540a        | Little Boy 1 (City; Alt Color Version 1)
15544 | chr0541         | Little Boy 2 (Rural)
15545 | chr0541a        | Little Boy 2 (Rural; Alt Color Version 1)
15546 | chr0548         | Male Middle Schooler 1 (City)
15547 | chr0549         | Male Middle Schooler 2 (City)
15548 | chr0550         | Young Man 1 (Eastern)
15549 | chr0550a        | Young Man 1 (Eastern; Alt Color Version 1) | btlface0_c0550a
15550 | chr0551         | Young Man 2 (Eastern) | btlface0_c0551
15551 | chr0551_c01     | Young Man 2 (Eastern; Hair/Color Update 1)
15552 | chr0551a        | Young Man 2 (Eastern; Alt Color Version 1)
15553 | chr0560         | Middle-Aged Man 1 (Eastern)
15554 | chr0560a        | Middle-Aged Man 1 (Eastern; Alt Color Version 1)
15555 | chr0561         | Middle-Aged Man 2 (Eastern)
15556 | chr0561_c01     | Middle-Aged Man 2 (Eastern; Fat Version)
15557 | chr0561a        | Middle-Aged Man 2 (Eastern; Alt Color Version 1)
15558 | chr0570         | Old Man 1 (Eastern)
15559 | chr0570_c01     | Old Man 1 (Eastern; Hair Changed to Black/Skin Color Changed Version)
15560 | chr0570a        | Old Man 1 (Eastern; Alt Color Version 1)
15561 | chr0580         | Young Boy 1 (Eastern)
15562 | chr0580a        | Young Boy 1 (Eastern; Alt Color 1)
15563 | chr0590         | Little Boy 1 (Eastern)
15564 | chr0600         | Young Man 1 (Central Eastern)
15565 | chr0600a        | Young Man 1 (Central Eastern; Alt Color Version 1)
15566 | chr0601         | Young Man 2 (Central Eastern)
15567 | chr0601a        | Young Man 2 (Central Eastern; Alt Color Version 1)
15568 | chr0610         | Middle-Aged Man 1 (Central Eastern)
15569 | chr0610_c01     | Middle-Aged Man 1 (Central Eastern; Fat Version)
15570 | chr0610a        | Middle-Aged Man 1 (Central Eastern; Alt Color Version 1)
15571 | chr0611         | Middle-Aged Man 2 (Central Eastern)
15572 | chr0611a        | Middle-Aged Man 2 (Central Eastern; Alt Color Version 1)
15573 | chr0620         | Old Man 1 (Central Eastern)
15574 | chr0620_c01     | Old Man 1 (Central Eastern; Hairstyle/Color Changed)
15575 | chr0630         | Young Boy 1 (Central Eastern)
15576 | chr0630a        | Young Boy 1 (Central Eastern; Alt Color Version 1)
15577 | chr0640         | Little Boy 1 (Central Eastern)
15578 | chr0652         | Caucasian Priest 1 (Young Man)
15579 | chr0652_c01     | Caucasian Priest 2 (Young Man)
15580 | chr0653         | Eastern Priest 1 (Young Man)
15581 | chr0654         | Central Eastern Priest 1 (Young Man)
15582 | chr0655         | Caucasian Priest 1 (Middle-Aged Man)
15583 | chr0656         | Eastern Priest 1 (Middle-Aged Man)
15584 | chr0657         | Central Eastern Priest 1 (Middle-Aged Man)
15585 | chr0658         | Archbishop 1 (Old Man)
15586 | chr0660         | Republic Police Officer 1 (Young Man)
15587 | chr0660_c01     | Republic Police Officer 2 (Young Man)
15588 | chr0661         | Republic Police Officer 1 (Middle-Aged Man)
15589 | chr0661_c01     | Republic Police Officer 2 (Middle-Aged Man)
15590 | chr0665         | Young Man Merchant 1
15591 | chr0666         | Middle-Aged Man Merchant 1
15592 | chr0667         | Old Man Merchant 1
15593 | chr0668         | Young Man Laborer 1
15594 | chr0668_c01     | Young Man Laborer 2
15595 | chr0669         | Young Man Laborer 1
15596 | chr0669a        | Young Man Laborer 1 (Skin and Hair Color Changed to Central Eastern 1)
15597 | chr0670         | Young Man Engineer 1
15598 | chr0670a        | Young Man Engineer 1 (Skin and Hair Color Changed to Eastern 1)
15599 | chr0671         | Middle-Aged Man Engineer 1
15600 | chr0700         | Caucasian Young Adult Chef 1
15601 | chr0701         | Eastern Young Adult Chef 1
15602 | chr0702         | Central Eastern Young Adult Chef 1
15603 | chr0703         | Caucasian Middle-Aged Chef 1
15604 | chr0704         | Eastern Middle-Aged Chef 1
15605 | chr0705         | Central Eastern Middle-Aged Chef 1
15606 | chr0705_c01     | Central Eastern Middle-Aged Chef 1 (Middle-Aged Man; Fat Version)
15607 | chr0710         | Young Man 1 (Suit)
15608 | chr0710a        | Young Man 1 (Suit; Alt Color Version 1)
15609 | chr0710b        | Young Man 1 (Suit; Alt Skin Color Version 1; Eastern)
15610 | chr0711         | Young Man 2 (Suit; No Jacket)
15611 | chr0711a        | Young Man 2 (Suit; No Jacket; Alt Color Version 1)
15612 | chr0711b        | Young Man 2 (Suit; No Jacket; Alt Skin Color Version 1)
15613 | chr0712         | Young Man 3 (Vest) | btlface0_c0712
15614 | chr0712a        | Young Man 3 (Vest; Alt Color Version 1) | btlface0_c0712a
15615 | chr0712b        | Young Man 3 (Vest; Alt Skin Color Version 1; Central Eastern)
15616 | chr0715         | Middle-Aged Man 1 (Suit)
15617 | chr0715a        | Middle-Aged Man 1 (Suit; Alt Color Version 1)
15618 | chr0715b        | Middle-Aged Man 1 (Suit; Alt Color Version 2)
15619 | chr0716         | Middle-Aged Man 2 (Suit; No Jacket)
15620 | chr0716_c01     | Middle-Aged Man 2 (Suit; No Jacket; Fat Version)
15621 | chr0716_c01a    | Middle-Aged Man 2 (Suit; No Jacket; Eastern)
15622 | chr0716_c02     | Middle-Aged Man 2 (Suit; No Jacket; Poker Use)
15623 | chr0716a        | Middle-Aged Man 2 (Suit; No Jacket; Alt Color Version 1)
15624 | chr0717         | Middle-Aged Man 3 (High Grade)
15625 | chr0717_c01     | Middle-Aged Man 3 (High Grade; Fat Version)
15626 | chr0717a        | Middle-Aged Man 3 (High Grade; Alt Color Version 1)
15627 | chr0717b        | Middle-Aged Man 3 (High Grade; Eastern)
15628 | chr0717c        | Middle-Aged Man 3 (High Grade; Central Eastern)
15629 | chr0720         | Old Man 1 (Suit)
15630 | chr0720_c01     | Old Man 1 (Suit; Old Man Huck Use)
15631 | chr0720a        | Old Man 1 (Suit; Alt Skin Color Version 1; Central Eastern)
15632 | chr0721         | Old Man 2 (Suit Vest)
15633 | chr0721a        | Old Man 2 (Suit Vest; Central Eastern)
15634 | chr0722         | Old Man 3 (High Grade)
15635 | chr0724         | Young Man 3 (Dealer)
15636 | chr0724_c01     | Middle-Aged Man 1 (Dealer)
15637 | chr0724_c02     | Middle-Aged Man 2 (Dealer)
15638 | chr0725         | Young Adult SP 1 (Suit) | btlface0_c0725
15639 | chr0725_c01     | Young Adult SP 2 (Suit) | btlface0_c0725_c01
15640 | chr0725_c02     | Young Adult SP 2 (Suit; SP Massive) | btlface0_c0725_c02
15641 | chr0725_c03     | Young Adult SP 2 (Suit; SP Massive) | btlface0_c0725_c02
15642 | chr0726         | Mafioso 1 | btlface0_c0726
15643 | chr0726_c01     | Mafioso 2 | btlface0_c0726_c01
15644 | chr0730         | Young Man Station Worker 1
15645 | chr0731         | Young Man Station Worker 1 (Alt Station Worker Color)
15646 | chr0735         | Young Man White Clothes 1
15647 | chr0735a        | Young Man White Clothes 1 (Alt Skin Color Version 1; Central Eastern)
15648 | chr0735_c01     | Young Man White Clothes 2
15649 | chr0735_c01a    | Young Man White Clothes 2 (Alt Skin Color Version 1; Central Eastern)
15650 | chr0740         | Middle-Aged Man White Clothes 1
15651 | chr0740_c01     | Middle-Aged Man White Clothes 2
15652 | chr0745         | Young Man Coat 1
15653 | chr0750         | Middle-Aged Man Coat 1 | btlface0_c0750
15654 | chr0750_c01     | Middle-Aged Man Coat 1 (No Hat)
15655 | chr0755         | Young Adult Soldier 1 | btlface0_c0755
15656 | chr0755_c01     | Young Adult Soldier 2
15657 | chr0756         | Young Adult Navy Sailor 1 (Navy Color)
15658 | chr0756_c01     | Young Adult Navy Sailor 2 (Navy Color)
15659 | chr0760         | Hercules (Rifle) | btlface0_c0760
15665 | chr0780         | Marduk Employee (Gun) (1) Security Guard Style | btlface0_c0780
15666 | chr0780_c01     | Marduk Employee (Gun) (2) Security Guard Style | btlface0_c0780_c01
15667 | chr0781         | Marduk Employee (Gun) (1) Serious Style | btlface0_c0781
15668 | chr0781_c01     | Marduk Employee (Gun) (2) Serious Style | btlface0_c0781_c01
15669 | chr0785         | Young Man Gangster 1 | btlface0_c0785
15670 | chr0786         | Young Man Gangster 2 | btlface0_c0786
15671 | chr0787         | Young Man Gangster 3 | btlface0_c0787
15672 | chr0790         | Jaeger Dropout 1 (Gun) | btlface0_c0790
15673 | chr0790_c01     | Jaeger Dropout 2 (Sword) | btlface0_c0790_c01
15674 | chr0795         | Young Man Heiyue Member 1 | btlface0_c0795
15675 | chr0795_c01     | Young Man Heiyue Member 2 | btlface0_c0795_c01
15678 | chr0805         | Male Student 1 (Year 1)
15679 | chr0805_c01     | Male Student 2 (Year 1)
15680 | chr0805_c02     | Male Student 3 (Year 2)
15681 | chr0805_c03     | Male Student 4 (Year ?)
15682 | chr0810         | Young Man Swimsuit 1
15683 | chr0811         | Young Man Swimsuit 2
15684 | chr0820         | Young Man Bath Getup 1
15685 | chr0821         | Middle-Aged Man Bath Getup 1
15686 | chr0822         | Old Man Bath Getup 1
15687 | chr0830         | Young Adult Bracer 1
15688 | chr0831         | Young Adult Bracer 2
15689 | chr0832         | Young Adult Bracer 3 | btlface0_c0832
15690 | chr0833         | Young Adult Bracer 4
15691 | chr0834         | Young Adult Bracer 5
15692 | chr0835         | Middle-Aged Bracer 1
15693 | chr0836         | Middle-Aged Bracer 2
15694 | chr0837         | Middle-Aged Bracer 3
15696 | chr0845         | Young Man Samue 1
15697 | chr0845_c01     | Young Man Samue 2
15698 | chr0846         | Middle-Aged Man Samue 1
15699 | chr0847         | Old Man Samue 1
15700 | chr0848         | Middle-Aged Man Samue (Ryokan Type) 1 (Hat)
15701 | chr0848_c01     | Middle-Aged Man Samue (Ryokan Type) 2
15702 | chr0849         | Old Man Samue (Ryokan Type) 1
15703 | chr0850         | Male Group Member 1 (Eastern Theater Troupe)
15704 | chr5500         | Young Woman 1 (City)
15705 | chr5500a        | Young Woman 1 (City; Alt Color Version 1)
15706 | chr5500b        | Young Woman 1 (City; Alt Color Version 2)
15707 | chr5501         | Young Woman 2 (City)
15708 | chr5501a        | Young Woman 2 (City; Alt Color Version 1)
15709 | chr5501b        | Young Woman 2 (City; Alt Color Version 2)
15710 | chr5501c        | Young Woman 2 (City; Eastern, Black Hair)
15711 | chr5502         | Young Woman 3 (City)
15712 | chr5502a        | Young Woman 3 (City; Alt Color Version 1)
15713 | chr5502b        | Young Woman 3 (City; Alt Color Version 2)
15714 | chr5503         | Young Woman 4 (Rural)
15715 | chr5503a        | Young Woman 4 (Rural; Alt Color Version 1)
15716 | chr5503b        | Young Woman 4 (Rural; Alt Color Version 2)
15717 | chr5504         | Young Woman 5 (City)
15718 | chr5504a        | Young Woman 5 (City; Alt Color Version 1)
15719 | chr5504b        | Young Woman 5 (City; Alt Color Version 2)
15720 | chr5510         | Middle-Aged Woman 1 (City)
15721 | chr5510a        | Middle-Aged Woman 1 (City; Alt Color Version 1)
15722 | chr5510b        | Middle-Aged Woman 1 (City; Alt Color Version 2)
15723 | chr5511         | Middle-Aged Woman 2 (City)
15724 | chr5511_c01     | Middle-Aged Woman 2 (City; Fat Version)
15725 | chr5511_c02     | Middle-Aged Woman 2 (City; Alt Hairstyle and Skin Color 1)
15726 | chr5511a        | Middle-Aged Woman 2 (City; Alt Color Version 1)
15727 | chr5511b        | Middle-Aged Woman 2 (City; Alt Color Version 2) | btlface0_c5511b
15728 | chr5513         | Middle-Aged Woman 4 (Rural)
15729 | chr5513_c01     | Middle-Aged Woman 4 (Rural; Fat Version)
15730 | chr5513a        | Middle-Aged Woman 4 (Rural; Alt Color Version 1; Brown Hair)
15731 | chr5513b        | Middle-Aged Woman 4 (Rural; Alt Color Version 2)
15732 | chr5520         | Old Woman 1 (City)
15733 | chr5520a        | Old Woman 1 (City; Alt Color Version 1)
15734 | chr5520b        | Old Woman 1 (City; Mayor Wen's Wife)
15735 | chr5521         | Old Woman 2 (Rural)
15736 | chr5522         | Old Woman 3 (Rural)
15737 | chr5522a        | Old Woman 3 (Rural; Alt Color Version 1)
15738 | chr5530         | Young Girl 1 (City)
15739 | chr5530a        | Young Girl 1 (City; Alt Color Version 1)
15740 | chr5531         | Young Girl 2 (City)
15741 | chr5531_c01     | Young Girl 2 (City; Alt Hairstyle and Skin Color 1; Wealthy)
15742 | chr5531a        | Young Girl 2 (City; Alt Color Version 1)
15743 | chr5532         | Young Girl 2 (City)
15744 | chr5532a        | Young Girl 2 (City; Alt Color Version 1)
15745 | chr5532b        | Young Girl 2 (City; Alt Skin Color Version 1; Central Eastern)
15746 | chr5540         | Little Girl 1 (City)
15747 | chr5540a        | Little Girl 1 (City; Alt Color Version 1)
15748 | chr5540b        | Little Girl 1 (City; Alt Skin Color Version 1; Eastern)
15749 | chr5541         | Little Girl 2 (Rural)
15750 | chr5541a        | Little Girl 2 (Rural; Alt Color Version 1)
15751 | chr5541b        | Little Girl 2 (Rural; Alt Skin Color Version 1; Eastern)
15752 | chr5550         | Young Woman 1 (Eastern)
15753 | chr5550a        | Young Woman 1 (Eastern; Alt Color Version 1)
15754 | chr5551         | Young Woman 2 (Eastern)
15755 | chr5551a        | Young Woman 2 (Eastern; Alt Color Version 1)
15756 | chr5560         | Middle-Aged Woman 1 (Eastern)
15757 | chr5560a        | Middle-Aged Woman 1 (Eastern; Alt Color Version 1)
15758 | chr5561         | Middle-Aged Woman 2 (Eastern)
15759 | chr5561_c01     | Middle-Aged Woman 2 (Eastern; Fat Version)
15760 | chr5561a        | Middle-Aged Woman 2 (Eastern; Alt Color Version 1)
15761 | chr5570         | Old Woman 1 (Eastern)
15762 | chr5570a        | Old Woman 1 (Eastern; Alt Color Version 1)
15763 | chr5570b        | Old Woman 1 (Eastern; Alt Color Version 1; Caucasian)
15764 | chr5580         | Young Girl 1 (Eastern)
15765 | chr5580a        | Young Girl 1 (Eastern; Alt Color Version 1)
15766 | chr5590         | Little Girl 1 (Eastern)
15767 | chr5590a        | Little Girl 1 (Eastern; Alt Color Version 1; Subdued?)
15768 | chr5600         | Young Woman (Central Eastern) 1
15769 | chr5600a        | Young Woman 1 (Central Eastern; Alt Color Version 1)
15770 | chr5601         | Young Woman 2 (Central Eastern)
15771 | chr5601a        | Young Woman 2 (Central Eastern; Alt Color Version 1)
15772 | chr5610         | Middle-Aged Woman 1 (Central Eastern)
15773 | chr5610a        | Middle-Aged Woman 1 (Central Eastern; Alt Color Version 1)
15774 | chr5611         | Middle-Aged Woman 2 (Central Eastern)
15775 | chr5611_c01     | Middle-Aged Woman 2 (Central Eastern; Fat Version)
15776 | chr5611a        | Middle-Aged Woman 2 (Central Eastern; Alt Color Version 1)
15777 | chr5620         | Old Woman 1 (Central Eastern)
15778 | chr5630         | Young Girl 1 (Central Eastern)
15779 | chr5630a        | Young Girl 1 (Central Eastern; Alt Color Version 1)
15780 | chr5640         | Little Girl 1 (Central Eastern)
15781 | chr5640a        | Little Girl 1 (Central Eastern; Alt Color Version 1)
15782 | chr5650         | Caucasian Nun 1 (Young Woman)
15783 | chr5650a        | Caucasian Nun 1 (Young Woman; Central Eastern)
15784 | chr5650_c01     | Caucasian Nun 2 (Young Woman)
15785 | chr5651         | Eastern Nun 1 (Young Woman)
15786 | chr5652         | Central Eastern Nun 1 (Young Woman)
15787 | chr5655         | Caucasian Nun 1 (Middle-Aged Woman)
15788 | chr5660         | Republic Police Officer 1 (Young Woman)
15789 | chr5660a        | Republic Police Officer 1 (Young Woman; Central Eastern; No Hat)
15790 | chr5660_c01     | Republic Police Officer 2 (Young Woman)
15791 | chr5670         | Suit 1 (Young Woman)
15792 | chr5670a        | Suit 1 (Young Woman; Alt Color Version 1)
15793 | chr5670b        | Suit 1 (Young Woman; Alt Skin Color Version 1; Eastern)
15794 | chr5670c        | Suit 1 (Young Woman; Alt Skin Color Version 2; Central Eastern)
15795 | chr5671         | Suit 2 (Young Woman)
15796 | chr5671_c01     | Suit 2 (Young Woman; Carol)
15797 | chr5671a        | Suit 2 (Young Woman; Alt Color Version 1)
15798 | chr5671b        | Suit 2 (Young Woman; Alt Skin Color Version 1; Central Eastern)
15799 | chr5675         | Suit 1 (Middle-Aged Woman)
15800 | chr5675_c01     | Suit 1 (Middle-Aged Woman; Alt Hairstyle Color 1)
15801 | chr5675a        | Suit 1 (Middle-Aged Woman; Alt Color Version 1)
15802 | chr5676         | Suit 2 (Middle-Aged Woman)
15803 | chr5676a        | Suit 2 (Middle-Aged Woman; Alt Color Version 1)
15804 | chr5676b        | Suit 2 (Middle-Aged Woman; Alt Skin Color Version 1; Eastern)
15805 | chr5680         | Young Woman 1 (Receptionist) | btlface0_c5680
15806 | chr5680a        | Young Woman 1 (Receptionist; Alt Color Version 1)
15807 | chr5680b        | Young Woman 1 (Receptionist; Alt Skin Color; Eastern)
15808 | chr5680_c01     | Young Woman 2 (Receptionist)
15809 | chr5680_c01a    | Young Woman 2 (Receptionist; Alt Skin Color; Central Eastern)
15810 | chr5682         | Young Woman 1 (Dealer)
15811 | chr5682a        | Young Woman 1 (Dealer; Central Eastern)
15812 | chr5695         | Caucasian Female Chef (Young Woman)
15813 | chr5695a        | Caucasian Female Chef (Young Woman; Eastern)
15814 | chr5700         | Female Student 1
15815 | chr5700_c01     | Female Student 2
15816 | chr5700_c02     | Female Student 3
15817 | chr5705         | Maid 1
15818 | chr5705a        | Maid 1 (Alt Skin Color 1; Eastern)
15819 | chr5706         | Waitress 1
15820 | chr5706a        | Waitress 1 (Alt Skin Color 1; Central Eastern)
15821 | chr5710         | Nightlife Dress Woman 1 (Young Woman) | btlface0_c5710
15822 | chr5710a        | Nightlife Dress Woman 1 (Young Woman; Alt Color Version 1)
15823 | chr5710b        | Nightlife Dress Woman 1 (Young Woman; Central Eastern)
15824 | chr5711         | Nightlife Dress Woman 2 (Young Woman) | btlface0_c5711
15825 | chr5711a        | Nightlife Dress Woman 2 (Young Woman; Alt Color Version 1)
15826 | chr5711b        | Nightlife Dress Woman 2 (Young Woman; Central Eastern)
15827 | chr5715         | Pole Dancer 1 (Young Woman)
15828 | chr5715a        | Pole Dancer 1 (Young Woman; Alt Color)
15829 | chr5715_c01     | Pole Dancer 2 (Young Woman)
15830 | chr5715_c01a    | Pole Dancer 2 (Young Woman; Alt Color)
15831 | chr5720         | White Clothes 1 (Young Woman)
15832 | chr5720_c01     | White Clothes 2 (Young Woman)
15833 | chr5720_c01a    | White Clothes 2 (Young Woman; Alt Skin Color 1)
15834 | chr5725         | White Clothes 2 (Middle-Aged Woman)
15835 | chr5730         | Coat 1 (Young Woman)
15836 | chr5730_c01     | Coat 2 (Young Woman; No Hat; Alt Clothes and Hair Color)
15837 | chr5735         | Coat 1 (Middle-Aged Woman)
15838 | chr5735_c01     | Coat 2 (Middle-Aged Woman; No Hat; Alt Clothes and Hair Color)
15839 | chr5740         | Swimsuit Female NPC 1 (Young Woman)
15841 | chr5741         | Swimsuit Female NPC 2 (Young Woman)
15843 | chr5745         | Bath Getup Female NPC 1 (Young Woman)
15844 | chr5746         | Bath Getup Female NPC 1 (Middle-Aged Woman)
15845 | chr5750         | Female Bracer 1 (Young Adult)
15846 | chr5751         | Female Bracer 2 (Young Adult) | btlface0_c5751
15847 | chr5752         | Female Bracer 2 (Middle-Aged)
15848 | chr5760         | Young Woman Samue 1
15849 | chr5760_c01     | Young Woman Samue 2
15850 | chr5761         | Young Woman Samue 1
15851 | chr5770         | Young Woman Samue 1 (Hostess/Mikasa)
15852 | chr5782         | Group Member 1 (Middle-Aged Woman)
15857 | chr5795         | Young Adult Woman Engineer 1
15858 | chr5795_c01     | Young Adult Woman Engineer 2
15859 | chr5795_c02     | Young Adult Woman Engineer 3
15860 | etc0000         | Dog 1
15861 | etc0001         | Dog 2
15862 | etc0010         | Cat 1
15863 | etc0012         | Cat 2
15864 | etc0011         | Cat 3
15865 | etc0013         | Cat 4
15866 | etc0020         | Camel 1
15867 | etc0030         | Fox
15868 | mon0047         | Archaism
15869 | mon0009         | Monster (Wolf)
15870 | mon0050         | Demon Sphere
15871 | chr0785a        | Young Man Gangster 1 (Alt Color)
15872 | chr0786a        | Young Man Gangster 2 (Alt Color) | btlface0_c0786a
15873 | chr0787a        | Young Man Gangster 3 (Alt Color)
15874 | chr0791         | Jaeger Dropout Alt Color 1 (Gun) | btlface0_c0791
15875 | chr0791_c01     | Jaeger Dropout Alt Color 1 (Sword) | btlface0_c0791_c01
15876 | chr0520a        | Old Man 1 (City; Alt Color Version 1; Kind Eyes Version)
15877 | etc0020_c01     | Camel 2
15878 | mon0017_c00     | Monster (Large Bird)
15879 | chr0720a        | Old Man 1 (Suit; Alt Skin Color Version 1; Central Eastern; Kind Eyes Version)
15880 | etc0002         | Dog 3
15881 | etc0003         | Dog 4
15882 | etc0031         | Fox 2
15883 | chr5505         | Young Woman 6 (Rural)
15884 | chr5501_c01     | Young Woman 2 Alt Color and Alt Hair
15885 | chr5700_c03     | Female Student 4
15886 | chr5753         | Female Bracer 3 (Young Adult)
15887 | chr0720_c02     | Old Man 1 (Suit; Alt Hair)
15888 | chr0631         | Young Boy (Central Eastern) 2
15889 | chr0715c        | Middle-Aged Man 1 (Suit; Alt Color Version 3)
15890 | chr5710_c01     | Nightlife Dress Woman 1 (Young Woman; Alt Hair 1)
15891 | chr5711_c02     | Nightlife Dress Woman 2 (Young Woman; Alt Hair 1)
15892 | chr0712_c01     | Young Man 3 (Vest; Alt Clothes Color & Alt Hair Style/New For Kuro 2)
15893 | chr0715_c01     | Middle-Aged Man 1 (Suit; Beard and Hat)
15894 | chr0717_c02     | Middle-Aged Man 3 (Alt Clothes Color & Alt Hair Style/New For Kuro 2)
15895 | chr0785_c01     | Young Man Gangster 1 (Alt Hair Style 1) | btlface0_c0785_c01
15896 | chr0786_c01     | Young Man Gangster 2 (Alt Hair Style 1)
15897 | chr0710_c01     | Young Man 1 (Suit; Alt Clothes Color & Alt Hair Style/New For Kuro 2)
15898 | chr0711_c01     | Young Man 2 (Suit; No Jacket; Alt Clothes Color & Alt Hair Style/New For Kuro 2) | btlface0_c0711_c01
15899 | chr0715_c02     | Middle-Aged Man 1 (Suit; Alt Clothes Color & Alt Hair Style/New For Kuro 2)
15900 | chr0716_c03     | Middle-Aged Man 2 (Suit; No Jacket; Alt Clothes Color & Alt Hair Style/New For Kuro 2)
15901 | chr0860         | Gekka School Martial Artist (Young Man) 01
15902 | chr0860_c01     | Gekka School Martial Artist (Young Man) 02
15903 | chr0861         | Gekka School Martial Artist (Middle-Aged) 01
15904 | chr5805         | Gekka School Martial Artist (Young Woman)
15905 | chr0805_c04     | Male Student 5 (Edmond) First Year
15906 | chr0805_c05     | Male Student 6 (Woody) Second Year
15907 | chr0805_c06     | Male Student 7 (Donatello) Second Year
15908 | chr0805_c07     | Male Student 8 (Elijah) Third Year
15909 | chr0805_c08     | Male Student 9 (Fully Generic 02) Caucasian First Year
15910 | chr0805_c09     | Male Student 10 (Fully Generic 03) Eastern First Year
15911 | chr0805_c10     | Male Student 11 (Fully Generic 04) Caucasian Second Year
15912 | chr0805_c11     | Male Student 12 (Fully Generic 05) Eastern Second Year
15913 | chr0805_c12     | Male Student 13 (Fully Generic 06) Caucasian Third Year
15914 | chr0806         | Male Student 9 (Jersey) Caucasian
15915 | chr0806_c01     | Male Student 12 (Jersey) Central Eastern
15916 | chr0807         | Male Student (Button Up Shirt and Apron) Edmond
15917 | chr0807_c01     | Male Student 13 (Button Up Shirt and Apron) Caucasian
15918 | chr0807_c02     | Male Student 10 (Button Up Shirt and Apron) Eastern
15919 | chr0808         | Male Student Samue
15920 | chr5700_c04     | Female Student 5 (Thelma) First Year
15921 | chr5700_c05     | Female Student 6 (Sigourney) First Year
15922 | chr5700_c06     | Female Student 7 (Aija) Second Year
15923 | chr5700_c07     | Female Student 8 (Frederica) Third Year
15924 | chr5700_c08     | Female Student 9 (Fully Generic 03) Central Eastern First Year
15925 | chr5700_c09     | Female Student 10 (Fully Generic 04) Caucasian Second Year
15926 | chr5700_c10     | Female Student 11 (Fully Generic 05) Caucasian Third Year
15927 | chr5700_c11     | Female Student 12 (Fully Generic 06) Central Eastern Second Year
15928 | chr5701         | Female Student 10 (Jersey) Caucasian
15929 | chr5701_c01     | Female Student 12 (Jersey) Eastern
15930 | chr5702         | Female Student Samue (Sigourney)
15931 | chr5702_c01     | Female Student Samue (Central Eastern)
15932 | chr5703         | Female Student Maid 1 (Caucasian)
15933 | chr5703_c01     | Female Student Maid 2 (Eastern)
15934 | chr0805_c13     | Male Student 14 (Fully Generic 07) Caucasian First-Year
15935 | chr5700_c12     | Female Student 13 (Fully Generic 07) Caucasian First-Year
15936 | chr5800         | Pit Girl 1 (ETWS/Leslie)
15937 | chr5801         | Pit Girl 2 (Red/Linmei)
15938 | chr5802         | Pit Girl 3 (Leno)
15939 | chr5803         | Pit Girl 4 (Ingert)
15940 | chr0855         | Z1 Racer Generic (ETWS)
15941 | chr0856         | Z1 Racer Generic (Ingert)
15942 | chr0857         | Z1 Racer Generic (Red Star)
15943 | chr0858         | Z1 Racer Generic (Leno)
19400 | chr5500         | Young Woman
19401 | chr0500         | Young Man | btlface0_c0500
19402 | chr5520         | Old Woman
19403 | chr0510         | Middle-Aged Man
19404 | chr5710         | Hostess | btlface0_c5710
19405 | chr5520         | Old Woman A
19406 | chr0510         | Middle-Aged Man A
19407 | chr5007_c03     | Grimcat | avfc5007_c03
19408 | chr0520         | Old Man A
19410 | chr0500         | Young Man A | btlface0_c0500
19411 | chr0501         | Young Man B | btlface0_c0501
19412 | chr0502         | Young Man C
19413 | chr5500         | Young Woman A
19414 | chr5113         | Yin | avfc5113_c00
19415 | chr0000         | Message
19416 | chr0785         | Gangster | btlface0_c0785
19417 | chr5561_c01     | Middle-Aged Woman
19418 | chr5551a        | Young Woman
19419 | chr0561_c01     | Middle-Aged Man
19420 | chr0670         | Man Dressed As Construction Worker
19797 | chr9990         | Dummy
20001 | veh0000         | Pickup Truck
20002 | vcar072         | Land Cruiser
20003 | vcar020         | Sedan
20004 | vcar051         | Limousine
20005 | vcar061         | Truck
20006 | vcar080         | Bus
20007 | vcar003         | Pickup Truck
20010 | vtrain020       | Tram
20011 | vcar080         | Orbal Bus
20012 | vcar080         | Orbal Bus
20013 | vbike010        | Motorcycle (Walter)
20014 | vcar070         | Land Carrier (Alvis)
20015 | vcar000         | Pickup Truck
20016 | vcar070         | Jeep
20017 | vcar040         | Light 1
20018 | vcar080         | Orbal Bus
20019 | vcar030         | Sports Car 1
20020 | vcar020         | Sedan 1
20021 | vcar081         | Orbal City Bus
20022 | vcar051         | Limousine 2
20023 | vcar021         | Sedan 2
20024 | vcar062         | Mid-Size Truck
20025 | t20evt00        | Parade Vehicle
20026 | vtrain002       | Subway (4 Cars)
20029 | vcar020         | Sedan 1 (Leno)
20030 | vcar020b        | Sedan 1 (Leno; Busted Gold)
20031 | vcar020c        | Sedan 1 (Leno; Street Racer Colors)
20032 | vcar020d        | Sedan 1 (Leno; Gold)
20033 | vcar021         | Sedan 2 (Red Star)
20034 | vcar021b        | Immaculate Sedan 2 (Red Star)
20035 | vcar022         | Sedan 3 (ETWS)
20036 | vcar023         | Sedan Orbal Vehicle (Alt Color; Red Star)
20037 | vcar024         | Sedan 3 (3 Row Version; ETWS)
20038 | vcar025         | Taxi (ETWS; Alt Color)
20039 | vcar025b        | Taxi 2 (ETWS; Alt Color)
20040 | vcar026         | Patrol Car (Red Star; Alt Color)
20041 | vcar030         | Sports Car 1 (ETWS)
20042 | vcar031         | Sports Car 2 (Ingert)
20043 | vcar032         | Ultra High-End Sports Car (Alt Color)
20044 | vcar033         | Damaged Ultra High-End Sports Car (Alt Color)
20045 | vcar040         | Light 1 (Leno)
20046 | vcar041         | Light 2 (Red Star)
20047 | vcar042         | Subcompact (Alt Color)
20048 | vcar050         | Limousine 1 (ETWS)
20049 | vcar051         | Limousine 2 (ETWS)
20050 | vcar052         | Silver White Limousine (ETWS)
20051 | vcar053         | Black High-End Vehicle (Alt Color; Ingert)
20052 | vcar060         | Light Truck (Leno)
20053 | vcar061         | Small Truck (Ingert)
20054 | vcar062         | Mid-Size Truck (Red Star)
20055 | vcar063         | Display Vehicle (Renovated Small Truck)
20056 | vcar064         | Food Truck (Leno)
20057 | vcar065         | Eisenschild Vehicle (Mid-Size Truck; Alt Color)
20058 | vcar070         | Jeep (Leno)
20059 | vcar071         | Kruga Vehicle (Jeep; Alt Color)
20060 | vcar072         | Jeep 3 (Leno)
20061 | vcar073         | Military Jeep (Leno)
20062 | vcar080         | Orbal Long Distance Bus (Ingert; Long)
20063 | vcar081         | Orbal City Bus (Ingert; Short)
20064 | vcar082         | Langport-Colored City Bus
20065 | vcar083         | Film Crew Orbal Vehicle (Alt Color)
20066 | vcar090         | Republic Tank
20067 | vcar100         | Z1 Car (Ingert)
20068 | vcar022b        | Immaculate Sedan 2 (Red Star)
20069 | vcar034         | Sports Car 1 (ETWS; Alt Color)
20070 | etc0200_c00     | Street Radio
20071 | e03kmo14        | Street Radio
20072 | e03kmo14        | Street Radio
20073 | g04isu04        | Generic Chair
20074 | vtrain020b      | Tram
20075 | etc0200         | Robin
20076 | etc0200_c00     | Sparrow
20077 | etc0201         | Crab
20078 | etc0201_c00     | Crab (Kazami Style)
20079 | etc0202         | Blue Swallowtail
20080 | etc0202_c00     | Old World Swallowtail
20081 | etc0203         | European Tortoise
20082 | etc0203_c00     | Texas Tortoise
20083 | etc0204         | Carp (Red and White)
20084 | etc0204_c00     | Carp (Red, White, and Black)
20085 | etc0204_c01     | Carp (White)
20086 | t10kmo17        | Goldfish (Red, with Frill)
20087 | t10kmo18        | Goldfish (Red)
20088 | t10kmo18b       | Goldfish (Black)
20089 | t10kmo18c       | Goldfish (Yellow)
42100 | fis0000         | ◆Kasagin
42101 | fis0001         | ◆Sky Gill
42102 | fis0002         | ◆Rainbow Trout
42103 | fis0003         | ◆Carp
42104 | fis0004         | ◆Snow Crab
42105 | fis0005         | ◆Red Perch
42106 | fis0006         | ◆Puffer
42107 | fis0007         | ◆Elecantus
42108 | fis0008         | ◆Rockeater
42109 | fis0009         | ◆Tiger Arowana
42110 | fis0010         | ◆Cangiru
42111 | fis0011         | ◆Pink Azelfish
42112 | fis0012         | ◆Quicksilver Carp
42113 | fis0013         | ◆Forest Gill
42114 | fis0014         | ◆Noble Carp
42115 | fis0015         | ◆Zafiro Espada
42116 | fis0016         | ◆Pearlglass
42117 | fis0017         | ◆Gold Salmon
42118 | fis0018         | ◆Snow White
42119 | fis0019         | ◆Flamear Ojo
42120 | fis0020         | ◆Snakehead
42121 | fis0021         | ◆Estrella
42122 | fis0022         | ◆Spiky Puffer
42123 | fis0023         | ◆Glutton Shark
42124 | fis0024         | ◆Blue Marlin
42125 | fis0025         | ◆Northern Arowana
42126 | fis0026         | ◆Cobalt Crab
42127 | fis0027         | ◆The Boss-Fish
42130 | fis9999         | ◆For Test
49990 | chr0120_c03     | Test Character 0 (Mare)
49991 | chr0000_c62     | Test Character 1 (Costume Van Test)
49992 | chr0122         | Test Character 2 (Ixs Hair Wave Adjustment)
49993 | chr5120         | Test Character 3 (Jorda Hair Wave Adjustment)
49999 | chr0000         | Test Character (Combat Use)
65400 | xipha           | Xipha
65401 | xipha_screen    | Spectrum
65535 | chr0298         | Overlord Auguste
65535 | chr0299         | Overlord Dantès
65535 | rob0002         | Assault Frame Xycres
65535 | rob0002_c00     | Assault Frame Xycres_Silver
65535 | rob0002_c01     | Assault Frame Xycres_Armed Red Eyes
65535 | rob0002_c02     | Assault Frame Xycres_President Machine
65535 | rob0002_c90     | Assault Frame Xycres_Major Damage
65535 | rob0002_c03     | Assault Frame Xycres_Military Armament
65535 | veh0000         | Pickup Truck
65535 | veh0001         | Upgraded Pickup Truck
65535 | veh0002         | Maxim's Sports Car
65535 | veh0003         | Lucio's Sports Car
65535 | veh0100         | Z1 Car_Red
65535 | veh0101         | Z1 Car_Green
65535 | veh0102         | Z1 Car_Light Blue
65535 | veh5000         | Orbal Bike (Red Star)
65535 | veh5001         | Large Orbal Bike (Etois)
65535 | veh5002         | Courier Bike
65535 | veh5003         | Generic NPC Bike
65535 | veh5004         | Van's Bike
65535 | etc0309         | Ixs & Jorda's Fusion Gun
65535 | chr0318_c03     | Maxim (Racing Suit: Helmet Off)
65535 | chr0318_c04     | Maxim (Racing Suit: Helmet On)
65535 | chr0326         | Tausend
65535 | chr0326_c00     | Tausend (Button-up Shirt)
65535 | chr0328_c00     | Vice Chief Hasan (For Event, Injured)
65535 | chr0333_c00     | Man in a Black Coat (Identity Revealed)
65535 | chr5311_c00     | Yumé (Different Outfit)
65535 | chr0000_c00     | Van (No Jacket)
65535 | chr0000_c01     | Van
65535 | chr0000_c07     | Van (Samue)
65535 | chr0000_c02     | Van (Bath Getup)
65535 | chr0000_c08     | Van (Shirtless)
65535 | chr0000_c09     | Van (Shirtless and Bandaged)
65535 | chr0000_c10     | Van (Racing Suit & Helmet)
65535 | chr0000_c48     | Van (Swimsuit/No Sandals)
65535 | chr0000_c55     | Van (Swimsuit DLC/Jacket)
65535 | chr0000_c56     | Van (Swimsuit DLC/No Jacket)
65535 | chr0000_c63     | Van (DLC Suit)
65535 | chr0002_c02     | Aaron
65535 | chr0002_c04     | Aaron Home Outfit
65535 | chr0002_c05     | Aaron Theater Costume
65535 | chr0002_c06     | Aaron Theater Costume_No Wig
65535 | chr0002_c07     | Aaron (Samue)
65535 | chr0002_c08     | Aaron (Shirtless)
65535 | chr0002_c09     | Aaron (Shirtless and Bandaged)
65535 | chr0002_c55     | Aaron (Swimsuit DLC/Jacket)
65535 | chr0002_c56     | Aaron (Swimsuit DLC/No Jacket)
65535 | chr0002_c63     | Aaron (DLC Suit)
65535 | chr0004_c00     | Quatre (No Jacket)
65535 | chr0004_c02     | Quatre (Bath Getup)
65535 | chr0004_c07     | Quatre (Samue)
65535 | chr0004_c08     | Quatre (For Event No Headphones)
65535 | chr0004_c09     | Quatre (Female Avatar Version)
65535 | chr0004_c49     | Quatre (Angel)
65535 | chr0004_c48     | Quatre (Swimsuit/No Sandals)
65535 | chr0004_c55     | Quatre (Swimsuit DLC/Jacket)
65535 | chr0004_c56     | Quatre (Swimsuit DLC/No Jacket)
65535 | chr0004_c58     | Quatre (DLC/Aramis Middle School Uniform)
65535 | chr0004_c63     | Quatre (DLC Suit)
65535 | chr0006_c00     | Bergard (No Jacket)
65535 | chr0006_c02     | Bergard (Bath Getup)
65535 | chr0006_c05     | Bergard (Goggles)
65535 | chr0006_c55     | Bergard (Swimsuit DLC/Jacket)
65535 | chr0006_c56     | Bergard (Swimsuit DLC/No Jacket)
65535 | chr0009_c00     | Van/Grendel Final Battle Spec
65535 | chr0111_c02     | Zin (Towel)
65535 | chr0112_c00     | Kincaid (Sleeves Rolled Up)
65535 | chr0112_c63     | Kincaid (DLC Suit)
65535 | chr0115_c00     | Melchior (Shard Clone)
65535 | chr0115_c01     | Melchior (Pupil Highlight and Soft Expression)
65535 | chr0115_c02     | Demon Melchior
65535 | chr0115_c03     | Melchior (Chest Wound/Normal Version)
65535 | chr0116_c00     | Dantès (Dealer Outfit)
65535 | chr0116_c01     | Dantès (Fatally Wounded)
65535 | chr0116_c02     | Dantès (Immortal)
65535 | chr0117_c00     | Arioch (Living Armor State)
65535 | chr0120_c01     | Swin (Swimsuit)
65535 | chr0120_c03     | Swin (Uniform)
65535 | chr0120_c12     | Swin (Jersey)
65535 | chr0120_c63     | Swin (DLC Suit)
65535 | chr0121_c00     | Gardenmaster (Hood & Mask)
65535 | chr0299_c00     | Overlord Dantès (Gorged Chest for Event)
65535 | chr0303_c00     | Security Chief Kasim (Regular Clothes)
65535 | chr0306_c00     | Leon (Wearing Overcoat)
65535 | chr0306_c63     | Leon (DLC Suit)
65535 | chr0312_c12     | Albert (Jersey)
65535 | chr0314_c00     | Inspector Daswani (Shot)
65535 | chr0424_c00     | Jimmy (Alternate Outfit)
65535 | chr0490_c00     | Faceless Assassin: Male (Normal Face)
65535 | chr0806         | Male Student 9 (Jersey) Caucasian
65535 | chr0806_c01     | Male Student 12 (Jersey) Central Eastern
65535 | chr0807         | Male Student (Button-Up Shirt and Apron) Edmond
65535 | chr0807_c01     | Male Student 13 (Button-Up Shirt and Apron) Caucasian
65535 | chr0807_c02     | Male Student 10 (Button-Up Shirt and Apron) Eastern
65535 | chr0807_c03     | Male Student 2 (Button-Up Shirt and Apron) Lyle
65535 | chr5001         | Agnès (Previous Game's Outfit)
65535 | chr5001_c00     | Agnès (No Coat)
65535 | chr5001_c02     | Agnès (Bath Getup)
65535 | chr5001_c03     | Agnès (Uniform)
65535 | chr5001_c04     | Agnès (Home Outfit)
65535 | chr5001_c07     | Agnès (Samue)
65535 | chr5001_c11     | Agnès (Second Game Outfit No Coat)
65535 | chr5001_c12     | Agnès (Jersey)
65535 | chr5001_c50     | Agnès (DLC/Store Bonus)
65535 | chr5001_c55     | Agnès (Swimsuit DLC/Shirt)
65535 | chr5001_c56     | Agnès (Swimsuit DLC/No Shirt)
65535 | chr5001_c60     | Agnès (Color Swap DLC)
65535 | chr5001_c61     | Agnès (DLC Pit Girl/Store Bonus)
65535 | chr5001_c64     | Agnès (DLC Pit Girl/For Sale)
65535 | chr5003_c04     | Feri (Light Home Outfit)
65535 | chr5003_c00     | Feri (Hood)
65535 | chr5003_c02     | Feri (Bath Getup)
65535 | chr5003_c07     | Feri (Samue)
65535 | chr5003_c50     | Feri (DLC/Store Bonus Sleeves)
65535 | chr5003_c51     | Feri (DLC/Store Bonus Sleeves)
65535 | chr5003_c55     | Feri (Swimsuit DLC/Jacket)
65535 | chr5003_c56     | Feri (Swimsuit DLC/No Jacket)
65535 | chr5003_c64     | Feri (DLC Pit Girl/For Sale)
65535 | chr5005_c02     | Risette (Bath Getup)
65535 | chr5005_c04     | Risette (Home Outfit/Pants)
65535 | chr5005_c05     | Risette (Left Arm and Right Foot Damaged)
65535 | chr5005_c06     | Risette (Undergoing Treatment)
65535 | chr5005_c07     | Risette (Samue)
65535 | chr5005_c08     | Risette (Right Arm Swapped Version)
65535 | chr5005_c09     | Risette (Home Outfit/Seamless Skin)
65535 | chr5005_c48     | Risette (Swimsuit/No Sandals)
65535 | chr5005_c50     | Risette (DLC/Store Bonus)
65535 | chr5005_c55     | Risette (Swimsuit DLC/Shirt)
65535 | chr5005_c56     | Risette (Swimsuit DLC/No Shirt)
65535 | chr5005_c64     | Risette (DLC Pit Girl/For Sale)
65535 | chr5007_c01     | Judith (Swimsuit)
65535 | chr5007_c02     | Judith (Bath Getup)
65535 | chr5007_c03     | Judith (Grimcat)
65535 | chr5007_c04     | Judith (Water Fall Ritual Outfit_Normal)
65535 | chr5007_c05     | Judith (Waterfall Ritual Outfit_Event Start and End)
65535 | chr5007_c06     | Judith (Long Hair)
65535 | chr5007_c07     | Judith (Samue)
65535 | chr5007_c08     | Judith (Transformation Craft Model)
65535 | chr5007_c09     | Judith (Home Outfit Gown)
65535 | chr5007_c10     | Judith (Gown+Grimcat)
65535 | chr5007_c11     | Judith (Gown+Grimcat+Cap)
65535 | chr5007_c55     | Judith (DLC Swimsuit/Shirt)
65535 | chr5007_c56     | Judith (DLC Swimsuit/No Shirt)
65535 | chr5007_c59     | Judith (Golden Costume DLC)
65535 | chr5007_c64     | Judith (DLC Pit Girl/For Sale)
65535 | chr5114_c01     | Renne Bright
65535 | chr5114         | Renne Bright
65535 | chr5114_c02     | Renne Bright
65535 | chr5114_c64     | Renne (DLC Pit Girl/For Sale)
65535 | chr5114_c65     | Renne (DLC Grimcat/Strategy Guide Bonus)
65535 | chr5117_c01     | Shizuna (Swimsuit)
65535 | chr5117_c10     | Shizuna (Personal Outfit)
65535 | chr5117_c11     | Shizuna (Personal Outfit/No Coat)
65535 | chr5117_c02     | Shizuna Rem Misurugi
65535 | chr5117_c61     | Shizuna (DLC Pit Girl/Store Bonus)
65535 | chr5117_c64     | Shizuna (DLC Pit Girl/For Sale)
65535 | chr5119_c01     | Nadia (Swimsuit)
65535 | chr5119_c03     | Nadia (Uniform)
65535 | chr5119_c12     | Nadia (Jersey)
65535 | chr5119_c64     | Nadia (DLC Pit Girl/For Sale)
65535 | chr5490_c00     | Faceless Assassin: Female (Normal Face)
65535 | chr5701         | Female Student 10 (Jersey) Caucasian
65535 | chr5701_c01     | Female Student 12 (Jersey) Eastern
65535 | chr5701_c02     | Female Student 13 (Jersey) Sasha
65535 | chr0302_c02     | Prince Sherid (Bath Getup)
65535 | chr0331_c00     | Rocksmith (No Hat)
65535 | chr0331_c01     | Rocksmith (Suit)
65535 | chr0319_c00     | Gien Lu
65535 | chr0305_c00     | Ashladd (Mouth Open)
65535 | chr0305         | Ashladd (Hood)
65535 | chr0312_c00     | Albert (Apron)
65535 | chr5110_c01     | Elaine (Swimsuit)
65535 | chr5110_c02     | Elaine (Towel)
65535 | chr5110_c11     | Elaine (Personal Outfit No Coat)
65535 | chr5110_c12     | Elaine (Wedding Dress)
65535 | chr5110_c61     | Elaine (DLC Pit Girl/Store Bonus)
65535 | chr5110_c64     | Elaine (DLC Pit Girl/For Sale)
65535 | chr5112_c02     | Fie (Bath Getup)
65535 | chr5113_c00     | Rixia (Lightweight Stage Outfit)
65535 | chr5113_c04     | Rixia (Personal Outfit)
65535 | chr5118_c01     | Lucrezia the Golden Butterfly (Kimono)
65535 | chr5118_c02     | Lucrezia the Golden Butterfly (Swimsuit)
65535 | chr5300_c02     | Naje (Bath Outfit)
65535 | chr5301_c00     | Aida
65535 | chr5301_c01     | Aida
65535 | chr5302_c00     | Celis (Wearing Overcoat)
65535 | chr5302_c64     | Celis (DLC Pit Girl/For Sale)
65535 | chr5311_c02     | Yumé (Bath Outfit)
65535 | chr5311_c07     | Yumé (Samue)
65535 | chr5315_c01     | Nina (Swimsuit)
65535 | chr5315_c02     | Nina (Bath Outfit)
65535 | chr5315_c03     | Nina (Disguise/Aniras Girls Academy Uniform)
65535 | chr5315_c04     | Nina (Disguise/Street Musician)
65535 | chr5315_c05     | Nina (Michelle Costume)
65535 | chr5317_c00     | Saara (Personal Outfit)
65535 | chr5317_c02     | Saara (Bath Getup)
65535 | chr5318_c00     | Shaheena (Personal Outfit)
65535 | chr5318_c02     | Shaheena (Bath Getup)
65535 | chr0403         | President Daltie
65535 | chr0404         | Lucio
65535 | chr0404_c00     | Lucio (Button-up Shirt)
65535 | chr0405_c00     | Jacobin (Disguise)
65535 | chr0405_c01     | Jacobin (Disguise 2: Mechanic)
65535 | chr0407         | Kyle (Fisherman)
65535 | chr0408         | Giordano
65535 | chr0409         | Director Lam
65535 | chr0410         | Researcher Lionel
65535 | chr0411         | Jess
65535 | chr0413         | Ronald Griffith
65535 | chr0413_c00     | Ronald Griffith (Disguise)
65535 | chr0414_c09     | Syd (Bandaged Head)
65535 | chr0415         | chr0503 Young Man 4 (Underwear Only Model)
65535 | chr0416         | chr0712 Young Man 3 (Underwear Only Model)
65535 | chr0417         | chr0725 Young Adult SP 1 (Underwear Only Model)
65535 | chr0418         | chr0725_c01 Young Adult SP 1 (Underwear Only Model)
65535 | chr0419         | Marché Bakery Owner?
65535 | chr0420         | Ralph (Bouldering Customer)
65535 | chr0421         | Aslan (Z1 Racer)
65535 | chr0421_c00     | Aslan (Z1 Racer No Helmet)
65535 | chr0422         | Master Tailong
65535 | chr0423         | Mr. Raffaello
65535 | chr0424         | Jimmy
65535 | chr0425         | Jed? (For Messeldam Quest)
65535 | chr0426         | Senator Ryan
65535 | chr0428         | Aaron's Father
65535 | chr0429         | Groom (Connection Event Background Character)
65535 | chr0430         | Curtis (Aramis Academy Student Uniform)
65535 | chr0431         | Lodi (Jersey)
65535 | chr0490         | Faceless Assassin: Male
65535 | chr5400         | Shopkeeper Woman
65535 | chr5401         | Courier (Helmet On)
65535 | chr5402         | Women's Fitness Instructor (Ms. Sonia)
65535 | chr5402_c01     | Jersey Selma
65535 | chr5402_c02     | Women's Fitness Instructor (Ms. Sonia) (Matchmaking Outfit)
65535 | chr5402_c03     | Betty (Bouldering Receptionist)
65535 | chr5406         | Shannon (Fisherwoman)
65535 | chr5407         | Makino (Fisherman)
65535 | chr5408         | Daswani's Daughter
65535 | chr5409_c01     | Lila (Academy Festival Maid Outfit)
65535 | chr5410         | Kisara (Graffiti Art)
65535 | chr5410_c01     | Kisara (Aramis School Uniform)
65535 | chr5411         | Meixin
65535 | chr5412         | Binsui
65535 | chr5413         | Ms. Leifa
65535 | chr5490         | Faceless Assassin: Female
65535 | chr5414         | Ms. Matilda
65535 | chr5415         | Risette
65535 | chr5416         | Connection Event Bride
65535 | chr0500         | Young Man 1
65535 | chr0500a        | Young Man 1 Alt Color 1
65535 | chr0500b        | Young Man 1 Alt Color 2
65535 | chr0501         | Young Man 2
65535 | chr0501a        | Young Man 2 Alt Color 1
65535 | chr0501b        | Young Man 2 Alt Color 2
65535 | chr0502         | Young Man 3
65535 | chr0502a        | Young Man 3 Alt Color 1
65535 | chr0502b        | Young Man 3 Alt Color 2
65535 | chr0503         | Young Man 4
65535 | chr0503a        | Young Man 4 Alt Color 1
65535 | chr0503b        | Young Man 4 Alt Color 2
65535 | chr0504         | Young Man 5
65535 | chr0505         | Young Man 6
65535 | chr0510         | Middle-Aged Man 1
65535 | chr0510_c01     | Middle-Aged Man 1 Fat
65535 | chr0510a        | Middle-Aged Man 1 Alt Color 1
65535 | chr0510b        | Middle-Aged Man 1 Alt Color 2
65535 | chr0510c        | Middle-Aged Man 1 Alt Color 3
65535 | chr0511         | Middle-Aged Man 2
65535 | chr0511a        | Middle-Aged Man 2 Alt Color 1
65535 | chr0511b        | Middle-Aged Man 2 Alt Color 2
65535 | chr0512         | Middle-Aged Man 3
65535 | chr0512a        | Middle-Aged Man 3 Alt Color 1
65535 | chr0512b        | Middle-Aged Man 3 Alt Color 2
65535 | chr0513         | Middle-Aged Man 4
65535 | chr0513_c01     | Middle-Aged Man 4 Fat
65535 | chr0513a        | Middle-Aged Man 4 Alt Color 1
65535 | chr0513b        | Middle-Aged Man 4 Alt Color 2
65535 | chr0513c        | Middle-Aged Man 4 Alt Color 3
65535 | chr0514         | Middle-Aged Man 5
65535 | chr0514_c01     | Middle-Aged Man 5 Combination 1
65535 | chr0520         | Old Man 1
65535 | chr0520_c01     | Old Man 1 No Hat
65535 | chr0520a        | Old Man 1 Alt Color 1
65535 | chr0521         | Old Man 2
65535 | chr0521a        | Old Man 2 Alt Color 1
65535 | chr0530         | Young Boy 1
65535 | chr0530a        | Young Boy 1 Alt Color 1
65535 | chr0531         | Young Boy 2
65535 | chr0531a        | Young Boy 2 Alt Color 1
65535 | chr0532         | Young Boy 3
65535 | chr0532a        | Young Boy 3 Alt Color 1
65535 | chr0540         | Toddler 1
65535 | chr0540a        | Toddler 1 Alt Color 1
65535 | chr0541         | Toddler 2
65535 | chr0541a        | Toddler 2 Alt Color 1
65535 | chr0548         | Middle Schooler 1
65535 | chr0549         | Middle Schooler 2
65535 | chr0550         | Young Man (Eastern) 1
65535 | chr0550a        | Young Man (Eastern) 1 Alt Color 1
65535 | chr0550_c09     | Young Man (Eastern) 1 (Sei/Injured)
65535 | chr0551         | Young Man (Eastern) 2
65535 | chr0551a        | Young Man (Eastern) 2 Alt Color 1
65535 | chr0551_c01     | Young Man (Eastern) 2 Alt Hair 1
65535 | chr0560         | Middle-Aged Man (Eastern) 1
65535 | chr0560a        | Middle-Aged Man (Eastern) 1 Alt Color 1
65535 | chr0561         | Middle-Aged Man (Eastern) 2
65535 | chr0561_c01     | Middle-Aged Man (Eastern) 2 Fat
65535 | chr0561a        | Middle-Aged Man (Eastern) 2 Alt Color 1
65535 | chr0570         | Old Man (Eastern) 1
65535 | chr0570a        | Old Man (Eastern) 1 Alt Color 1
65535 | chr0570_c01     | Old Man (Eastern) 1 Alt Hair 1
65535 | chr0580         | Young Boy (Eastern) 1
65535 | chr0580a        | Young Boy (Eastern) 1 Alt Color 1
65535 | chr0590         | Toddler (Eastern) 1
65535 | chr0600         | Young Man (Central Eastern) 1
65535 | chr0600a        | Young Man (Central Eastern) 1 Alt Color 1
65535 | chr0601         | Young Man (Central Eastern) 2
65535 | chr0601a        | Young Man (Central Eastern) 2 Alt Color 1
65535 | chr0610         | Middle-Aged Man (Central Eastern) 1
65535 | chr0610_c01     | Middle-Aged Man (Central Eastern) 1 Fat
65535 | chr0610a        | Middle-Aged Man (Central Eastern) 1 Alt Color 1
65535 | chr0611         | Middle-Aged Man (Central Eastern) 2
65535 | chr0611a        | Middle-Aged Man (Central Eastern) 2 Alt Color 1
65535 | chr0620         | Old Man (Central Eastern) 1
65535 | chr0620_c01     | Old Man (Central Eastern) 1 Alt Hair 1
65535 | chr0630         | Young Boy (Central Eastern) 1
65535 | chr0630a        | Young Boy (Central Eastern) 1 Alt Color 1
65535 | chr0631         | Young Boy (Central Eastern) 2
65535 | chr0640         | Toddler (Central Eastern) 1
65535 | chr0652         | Young Priest (Caucasian) 1
65535 | chr0652_c01     | Young Priest (Caucasian) 2
65535 | chr0653         | Young Priest (Eastern) 1
65535 | chr0654         | Young Priest (Central Eastern) 1
65535 | chr0655         | Middle-Aged Priest (Caucasian) 1
65535 | chr0655_c01     | Middle-Aged Priest (Caucasian) 2
65535 | chr0656         | Middle-Aged Priest (Eastern) 1
65535 | chr0657         | Middle-Aged Priest (Central Eastern) 1
65535 | chr0658         | Old Bishop (Caucasian) 1
65535 | chr0660         | Young Policeman 1
65535 | chr0660_c01     | Young Policeman 2
65535 | chr0660_c01a    | Young Policeman 2 Fake Policeman (For Quest)
65535 | chr0660_c02     | Young Policeman 1 Dirty Clothes
65535 | chr0661         | Middle-Aged Policeman 1
65535 | chr0661_c01     | Middle-Aged Policeman 2
65535 | chr0665         | Young Adult Merchant 1
65535 | chr0665_c01     | Young Adult Merchant 2 (Devancy 3 Brothers)
65535 | chr0666         | Middle-Aged Merchant 1
65535 | chr0667         | Old Merchant 1
65535 | chr0668         | Young Adult Laborer 1
65535 | chr0668_c01     | Young Adult Laborer 2
65535 | chr0669         | Middle-Aged Laborer 1
65535 | chr0669a        | Middle-Aged Laborer 1 (Central Eastern)
65535 | chr0670         | Young Adult Engineer 1
65535 | chr0670a        | Young Adult Engineer 1 (Eastern)
65535 | chr0671         | Middle-Aged Engineer 1
65535 | chr0700         | Young Adult Chef (Caucasian) 1
65535 | chr0701         | Young Adult Chef (Eastern) 1
65535 | chr0702         | Young Adult Chef (Central Eastern) 2
65535 | chr0703         | Middle-Aged Chef (Caucasian) 1
65535 | chr0704         | Middle-Aged Chef (Eastern) 1
65535 | chr0705         | Middle-Aged Chef (Central Eastern) 1
65535 | chr0705_c01     | Middle-Aged Chef (Central Eastern) 1 Fat
65535 | chr0710         | Young Adult Suit 1
65535 | chr0710a        | Young Adult Suit 1 Alt Color 1
65535 | chr0710b        | Young Adult Suit 1 Alt Color 2
65535 | chr0710_c01     | Young Adult Suit 1 (Hair Style, Alt Color/New for Kuro 2)
65535 | chr0711         | Young Adult Suit 2
65535 | chr0711a        | Young Adult Suit 2 Alt Color 1
65535 | chr0711b        | Young Adult Suit 2 Alt Color 2
65535 | chr0711_c01     | Young Adult Suit 2 (Hair Style, Alt Color/New for Kuro 2)
65535 | chr0712         | Young Adult Suit 3
65535 | chr0712a        | Young Adult Suit 3 Alt Color 1
65535 | chr0712b        | Young Adult Suit 3 Alt Color 2
65535 | chr0712_c01     | Young Adult Suit 3 (Hair Style, Alt Color/New for Kuro 2)
65535 | chr0715         | Middle-Aged Suit 1
65535 | chr0715a        | Middle-Aged Suit 1 Alt Color 1
65535 | chr0715b        | Middle-Aged Suit 1 Alt Color 2
65535 | chr0715c        | Middle-Aged Suit 1 Alt Color 3
65535 | chr0715_c01     | Middle-Aged Suit 1 (No Hat)
65535 | chr0715_c02     | Middle-Aged Suit 1 (Hair Style, Alt Color/New for Kuro 2)
65535 | chr0716         | Middle-Aged Suit 2
65535 | chr0716_c01     | Middle-Aged Suit 2 Fat
65535 | chr0716_c01a    | Middle-Aged Suit 2 Fat (Central Eastern)
65535 | chr0716_c02     | Middle-Aged Suit 2 Alt Hair 1
65535 | chr0716a        | Middle-Aged Suit 2 Alt Color 1
65535 | chr0716_c03     | Middle-Aged Suit 2 (Hair Style, Alt Color/New for Kuro 2)
65535 | chr0717         | Middle-Aged Suit 3
65535 | chr0717_c01     | Middle-Aged Suit 3 Fat
65535 | chr0717a        | Middle-Aged Suit 3 Alt Color 1
65535 | chr0717b        | Middle-Aged Suit 3 Alt Color 2 (Eastern)
65535 | chr0717c        | Middle-Aged Suit 3 Alt Color 3 (Central Eastern)
65535 | chr0717_c02     | Middle-Aged Suit 3 (Hair Style, Alt Color/New for Kuro 2)
65535 | chr0720         | Old Suit 1
65535 | chr0720a        | Old Suit 1 Alt Color 1 (Central Eastern)
65535 | chr0720_c01     | Old Suit 1 Alt Hair 1
65535 | chr0720_c02     | Old Suit 1 Alt Hair 2
65535 | chr0721         | Old Suit 2
65535 | chr0721a        | Old Suit 2 Alt Color 1
65535 | chr0722         | Old Suit 3
65535 | chr0724         | Young Adult Dealer 1
65535 | chr0724_c01     | Middle-Aged Dealer 1
65535 | chr0724_c02     | Middle-Aged Dealer 2
65535 | chr0725         | SP1
65535 | chr0725_c01     | SP2
65535 | chr0725_c02     | SP1 (Massive Type)
65535 | chr0725_c03     | SP2 (Massive Type)
65535 | chr0726         | Mafia 1
65535 | chr0726_c01     | Mafia 2
65535 | chr0727         | Dolor Family 1
65535 | chr0727_c01     | Dolor Family 2
65535 | chr0730         | Young Adult Station Worker 1
65535 | chr0731         | Young Adult Attendant 1
65535 | chr0731_c01     | Young Adult Attendant 2 (Devancy 3 Brothers)
65535 | chr0735         | Young Adult White Clothes 1
65535 | chr0735a        | Young Adult White Clothes 1 Alt Color 1 (Central Eastern)
65535 | chr0735_c01     | Young Adult White Clothes 2
65535 | chr0735_c01a    | Young Adult White Clothes 2 Alt Color 1 (Central Eastern)
65535 | chr0740         | Middle-Aged White Clothes 1
65535 | chr0740_c01     | Middle-Aged White Clothes 2
65535 | chr0745         | Young Adult Coat 1
65535 | chr0750         | Middle-Aged Coat 1
65535 | chr0750_c01     | Middle-Aged Coat 1 (No Hat)
65535 | chr0755         | Young Adult Soldier 1
65535 | chr0755_c01     | Young Adult Soldier 2
65535 | chr0756         | Young Adult Navy Sailor 1
65535 | chr0756_c01     | Young Adult Navy Sailor 2
65535 | chr0757         | Middle-Aged Navy Sailor 1 (Devancy 3 Brothers)
65535 | chr0760         | Hercules 1
65535 | chr0760a        | Hercules 2 (Black)
65535 | chr0760b        | Hercules 3 (White)
65535 | chr0760c        | Hercules 4 (Black Full Face) For Prologue MAYA Event
65535 | chr0765         | Eisenschild 1
65535 | chr0765_c01     | Eisenschild 2
65535 | chr0770         | Kruga 1
65535 | chr0770_c01     | Kruga 2
65535 | chr0775         | Ikaruga 1
65535 | chr0780         | Marduk Security 1
65535 | chr0780_c01     | Marduk Security 2
65535 | chr0781         | Marduk Security EX 1
65535 | chr0781_c01     | Marduk Security EX 2
65535 | chr0783         | Zecht Arms Soldier 1
65535 | chr0783_c01     | Zecht Arms Soldier 2
65535 | chr0785         | Stylish Male Gangster 1
65535 | chr0785a        | Stylish Male Gangster 1 Alt Color 1
65535 | chr0785_c01     | Stylish Male Gangster 1 Alt Hair 1
65535 | chr0785_c02     | Stylish Male Gangster 1 (Beat Up)
65535 | chr0785_c03     | Stylish Male Gangster 1 (Zombie)
65535 | chr0785a_c03    | Stylish Male Gangster 1 Alt Color 1 (Zombie)
65535 | chr0785_c04     | Stylish Male Gangster 1 (Prologue Event EX)
65535 | chr0786         | Punk Male Gangster 2
65535 | chr0786a        | Punk Male Gangster 2 Alt Color 1
65535 | chr0786_c01     | Punk Male Gangster 2 Alt Hair 1
65535 | chr0786_c02     | Punk Male Gangster 2 (Beat Up)
65535 | chr0786_c03     | Punk Male Gangster 2 (Zombie)
65535 | chr0786a_c03    | Punk Male Gangster 2 Alt Color 1 (Zombie)
65535 | chr0786_c04     | Punk Male Gangster 2 (Prologue Event EX)
65535 | chr0787         | Laborer Male Gangster 3
65535 | chr0787a        | Laborer Male Gangster 3 Alt Color 1
65535 | chr0787_c02     | Laborer Male Gangster 3 (Beat Up)
65535 | chr0787_c03     | Laborer Male Gangster 3 (Zombie)
65535 | chr0787_c04     | Laborer Male Gangster 3 (Prologue Event EX)
65535 | chr0790_c01     | Jaeger Dropout 2
65535 | chr0790_c02     | Jaeger Dropout 1 (Helmet Cut, Face Half-Shown Version)
65535 | chr0790_c03     | Jaeger Dropout 2 (Pants, Lower Body Exposed Version)
65535 | chr0791         | Jaeger Dropout 1 Alt Color (Zahat's Scorpion) 1
65535 | chr0791_c01     | Jaeger Dropout 1 Alt Color (Zahat's Scorpion) 2
65535 | chr0792         | Anti-Immigrant Terrorist (Modified Jaeger Dropout)
65535 | chr0792_c01     | Anti-Immigrant Terrorist (Middle-Aged Commander) 1
65535 | chr0792_c02     | Anti-Immigrant Terrorist (Middle-Aged Commander) 2
65535 | chr0795         | Heiyue Member 1
65535 | chr0795_c01     | Heiyue Member 2
65535 | chr0800         | Heiyue Special Assassination Squad 1
65535 | chr0801         | Heiyue Special Assassination Squad 2
65535 | chr0805         | Male Student 1 (Julian)
65535 | chr0805_c01     | Male Student 2 (Lyle)
65535 | chr0805_c02     | Male Student 3 (James)
65535 | chr0805_c03     | Male Student 4 (Fully Generic 01)
65535 | chr0805_c04     | Male Student 5 (Edmond)
65535 | chr0805_c05     | Male Student 6 (Woody)
65535 | chr0805_c06     | Male Student 7 (Donatello)
65535 | chr0805_c07     | Male Student 8 (Elijah)
65535 | chr0805_c08     | Male Student 9 (Fully Generic 02) Caucasian
65535 | chr0805_c09     | Male Student 10 (Fully Generic 03) Eastern
65535 | chr0805_c10     | Male Student 11 (Fully Generic 04) Caucasian
65535 | chr0805_c11     | Male Student 12 (Fully Generic 05) Eastern
65535 | chr0805_c12     | Male Student 13 (Fully Generic 06) Caucasian
65535 | chr0805_c13     | Male Student 14 (Fully Generic 07) Caucasian First-Year
65535 | chr0808         | Male Student Samue
65535 | chr0809         | Male Student 8 (Elijah Personal Outfit)
65535 | chr0810         | Young Adult Swimsuit 1
65535 | chr0811         | Young Adult Swimsuit 2
65535 | chr0820         | Young Adult Bath Getup 1
65535 | chr0821         | Middle-Aged Bath Getup 1
65535 | chr0822         | Old Bath Getup 1
65535 | chr0830         | Young Adult Bracer 1
65535 | chr0831         | Young Adult Bracer 2
65535 | chr0831_c01     | Young Adult Bracer 2 (Fake Bracer)
65535 | chr0832         | Young Adult Bracer 3
65535 | chr0833         | Young Adult Bracer 4
65535 | chr0834         | Young Adult Bracer 5
65535 | chr0835         | Middle-Aged Bracer 1
65535 | chr0836         | Middle-Aged Bracer 2
65535 | chr0837         | Middle-Aged Bracer 3
65535 | chr0840         | Young Adult Knight 1
65535 | chr0845         | Young Adult Samue 1
65535 | chr0845_c01     | Young Adult Samue 2
65535 | chr0846         | Middle-Aged Samue 1
65535 | chr0847         | Old Samue 1
65535 | chr0848         | Kanza Middle-Aged Samue
65535 | chr0848_c01     | Mamiya Middle-Aged Samue
65535 | chr0849         | Hakurou Old Samue
65535 | chr0850         | Young Adult (Eastern Theater Troupe) 1
65535 | chr0850_c01     | Young Adult (Eastern Theater Troupe) 2
65535 | chr0855         | Z1 Racer Generic (ETWS)
65535 | chr0856         | Z1 Racer Generic (Ingert)
65535 | chr0857         | Z1 Racer Generic (Red Star)
65535 | chr0858         | Z1 Racer Generic (Leno)
65535 | chr0860         | Gekka School Martial Artist (Young Man) 01
65535 | chr0860_c01     | Gekka School Martial Artist (Young Man) 02
65535 | chr0861         | Gekka School Martial Artist (Middle-Aged) 01
65535 | chr0870         | Academy Festival Dancer (Eastern Outfit Julian)
65535 | chr0871         | Academy Festival Dancer (Western Outfit Dancer)
65535 | chr0872         | Academy Festival Dancer (Eastern Outfit Dancer)
65535 | chr0873         | Academy Festival Dancer (Central Eastern Outfit Dancer)
65535 | chr5500         | Young Woman 1
65535 | chr5500a        | Young Woman 1 Alt Color 1
65535 | chr5500b        | Young Woman 1 Alt Color 2
65535 | chr5501         | Young Woman 2
65535 | chr5501a        | Young Woman 2 Alt Color 1
65535 | chr5501b        | Young Woman 2 Alt Color 2
65535 | chr5501c        | Young Woman 2 Alt Color 3 (Eastern, Black Hair)
65535 | chr5501_c01     | Young Woman 2 Alt Color and Alt Hair
65535 | chr5502         | Young Woman 3
65535 | chr5502a        | Young Woman 3 Alt Color 1
65535 | chr5502b        | Young Woman 3 Alt Color 2
65535 | chr5503         | Young Woman 4
65535 | chr5503a        | Young Woman 4 Alt Color 1
65535 | chr5503b        | Young Woman 4 Alt Color 2
65535 | chr5504         | Young Woman 5
65535 | chr5504a        | Young Woman 5 Alt Color 1
65535 | chr5504b        | Young Woman 5 Alt Color 2
65535 | chr5505         | Young Woman 6
65535 | chr5510         | Middle-Aged Woman 1
65535 | chr5510a        | Middle-Aged Woman 1 Alt Color 1
65535 | chr5510b        | Middle-Aged Woman 1 Alt Color 2
65535 | chr5511         | Middle-Aged Woman 2
65535 | chr5511_c01     | Middle-Aged Woman 2 Fat 1
65535 | chr5511_c02     | Middle-Aged Woman 2 Alt Hair 1
65535 | chr5511a        | Middle-Aged Woman 2 Alt Color 1
65535 | chr5511b        | Middle-Aged Woman 2 Alt Color 2
65535 | chr5512         | Middle-Aged Woman 3
65535 | chr5513         | Middle-Aged Woman 4
65535 | chr5513_c01     | Middle-Aged Woman 4 Fat 1
65535 | chr5513a        | Middle-Aged Woman 4 Alt Color 1 Brown Hair
65535 | chr5513b        | Middle-Aged Woman 4 Alt Color 2
65535 | chr5520         | Old Woman 1
65535 | chr5520a        | Old Woman 1 Alt Color 1
65535 | chr5520b        | Old Woman 1 Alt Color 2
65535 | chr5521         | Old Woman 2
65535 | chr5522         | Old Woman 3
65535 | chr5522a        | Old Woman 3 Alt Color 1
65535 | chr5530         | Young Girl 1
65535 | chr5530a        | Young Girl 1 Alt Color 1
65535 | chr5531         | Young Girl 2
65535 | chr5531a        | Young Girl 2 Alt Color 1
65535 | chr5531_c01     | Young Girl 2 Alt Hair 1
65535 | chr5532         | Young Girl 3
65535 | chr5532a        | Young Girl 3 Alt Color 1
65535 | chr5532b        | Young Girl 3 Alt Color 2 (Eastern)
65535 | chr5540         | Little Girl 1
65535 | chr5540a        | Little Girl 1 Alt Color 1
65535 | chr5540b        | Little Girl 1 Alt Color 2
65535 | chr5541         | Little Girl 2
65535 | chr5541a        | Little Girl 2 Alt Color 1
65535 | chr5541b        | Little Girl 2 Alt Color 2 (Eastern)
65535 | chr5541_c01     | Little Girl 2 (Quest Hair Decorations Added)
65535 | chr5550         | Young Woman (Eastern) 1
65535 | chr5550a        | Young Woman (Eastern) 1 Alt Color 1
65535 | chr5551         | Young Woman (Eastern) 2
65535 | chr5551a        | Young Woman (Eastern) 2 Alt Color 1
65535 | chr5560         | Young Woman (Eastern) 1
65535 | chr5560a        | Young Woman (Eastern) 1 Alt Color 1
65535 | chr5561         | Young Woman (Eastern) 2
65535 | chr5561_c01     | Young Woman (Eastern) 2 Fat 1
65535 | chr5561a        | Young Woman (Eastern) 2 Alt Color 1
65535 | chr5570         | Old Woman (Eastern) 1
65535 | chr5570a        | Old Woman (Eastern) 1 Alt Color 1
65535 | chr5570b        | Old Woman (Eastern) 1 Alt Color 2 (Caucasion)
65535 | chr5580         | Young Girl (Eastern) 1
65535 | chr5580a        | Young Girl (Eastern) 1 Alt Color 1
65535 | chr5590         | Little Girl (Eastern) 1
65535 | chr5590a        | Little Girl (Eastern) 1 Alt Color 1 (Subdued Color)
65535 | chr5600         | Young Woman (Central Eastern) 1
65535 | chr5600a        | Young Woman (Central Eastern) 1 Alt Color 1
65535 | chr5601         | Young Woman (Central Eastern) 2
65535 | chr5601a        | Young Woman (Central Eastern) 2 Alt Color 1
65535 | chr5610         | Middle-Aged Woman (Central Eastern) 1
65535 | chr5610a        | Middle-Aged Woman (Central Eastern) 1 Alt Color 1
65535 | chr5611         | Middle-Aged Woman (Central Eastern) 2
65535 | chr5611_c01     | Middle-Aged Woman (Central Eastern) 2 Fat 1
65535 | chr5611a        | Middle-Aged Woman (Central Eastern) 2 Alt Color 1
65535 | chr5620         | Old Woman (Central Eastern) 1
65535 | chr5630         | Young Girl (Central Eastern) 1
65535 | chr5630a        | Young Girl (Central Eastern) 1 Alt Color 1
65535 | chr5630_c01     | Young Girl (Central Eastern) 1 Alt Hair 1
65535 | chr5640         | Little Girl (Central Eastern) 1
65535 | chr5640a        | Little Girl (Central Eastern) 1 Alt Color 1
65535 | chr5650         | Young Adult Sister (Caucasian) 1
65535 | chr5650_c01     | Young Adult Sister (Caucasian) 2
65535 | chr5650a        | Young Adult Sister (Caucasian) 1 (Central Eastern)
65535 | chr5651         | Young Adult Sister (Eastern) 1
65535 | chr5652         | Young Adult Sister (Central Eastern) 1
65535 | chr5655         | Young Adult Sister (Central Eastern) 1
65535 | chr5660         | Young Policewoman 1
65535 | chr5660a        | Young Policewoman 1 Alt Color 1 (Central Eastern)
65535 | chr5660a_c01    | Young Policewoman 1 Alt Color 1 (Central Eastern, Dirty Clothing)
65535 | chr5660_c01     | Young Policewoman 2
65535 | chr5670         | Young Woman Suit 1
65535 | chr5670a        | Young Woman Suit 1 Alt Color 1
65535 | chr5670b        | Young Woman Suit 1 Alt Color 2 (Eastern)
65535 | chr5670c        | Young Woman Suit 1 Alt Color 3 (Central Eastern)
65535 | chr5671         | Young Woman Suit 2
65535 | chr5671_c01     | Young Woman Suit 2 Alt Hair 1
65535 | chr5671a        | Young Woman Suit 2 Alt Color 1
65535 | chr5671b        | Young Woman Suit 2 Alt Color 2
65535 | chr5675         | Middle-Aged Woman Suit 1
65535 | chr5675_c01     | Middle-Aged Woman Suit 1 Alt Hair 1
65535 | chr5675a        | Middle-Aged Woman Suit 1 Alt Color 1
65535 | chr5676         | Middle-Aged Woman Suit 2
65535 | chr5676a        | Middle-Aged Woman Suit 2 Alt Color 1
65535 | chr5676b        | Middle-Aged Woman Suit 2 Alt Color 2
65535 | chr5680         | Young Woman Receptionist 1
65535 | chr5680a        | Young Woman Receptionist 1 Alt Color 1 (Blue Clothing)
65535 | chr5680b        | Young Woman Receptionist 1 Alt Color 2 (Eastern)
65535 | chr5680_c01     | Young Woman Receptionist 2
65535 | chr5680_c01a    | Young Woman Receptionist 2 Alt Color 1 (Central Eastern)
65535 | chr5682         | Young Woman Dealer 1
65535 | chr5682a        | Young Woman Dealer 1 Alt Color 1 (Central Eastern)
65535 | chr5695         | Young Woman Chef (Caucasian) 1
65535 | chr5695a        | Young Woman Chef (Caucasian) 1 Alt Color 1 (Eastern)
65535 | chr5700         | Female Student 1 (Mignon)
65535 | chr5700_c01     | Female Student 2 (Sasha)
65535 | chr5700_c02     | Female Student 3 (Fully Generic 01)
65535 | chr5700_c03     | Female Student 4 (Fully Generic 02)
65535 | chr5700_c04     | Female Student 5 (Thelma)
65535 | chr5700_c05     | Female Student 6 (Sigourney)
65535 | chr5700_c06     | Female Student 7 (Aija)
65535 | chr5700_c07     | Female Student 8 (Frederica)
65535 | chr5700_c08     | Female Student 9 (Fully Generic 03) Central Eastern
65535 | chr5700_c09     | Female Student 10 (Fully Generic 04) Caucasian
65535 | chr5700_c10     | Female Student 11 (Fully Generic 05) Caucasian
65535 | chr5700_c11     | Female Student 12 (Fully Generic 06) Central Eastern
65535 | chr5700_c12     | Female Student 13 (Fully Generic 07) Caucasian First-Year
65535 | chr5702         | Female Student Samue (Sigourney)
65535 | chr5702_c01     | Female Student Samue (Central Eastern)
65535 | chr5703         | Female Student Maid 1 (Caucasian)
65535 | chr5703_c01     | Female Student Maid 2 (Eastern)
65535 | chr5704         | Frederica (Personal Outfit)
65535 | chr5705         | Maid 1
65535 | chr5705a        | Maid 1 Alt Color 1 (Eastern)
65535 | chr5706         | Waitress 1
65535 | chr5706a        | Waitress 1 Alt Color 1 (Central Eastern)
65535 | chr5710         | Nightlife Dress 1
65535 | chr5710a        | Nightlife Dress 1 Alt Color 1
65535 | chr5710b        | Nightlife Dress 1 Alt Color 2
65535 | chr5710_c01     | Nightlife Dress 1 Alt Hair 1
65535 | chr5710_c02     | Nightlife Dress 1 Alt Hair 2
65535 | chr5711         | Nightlife Dress 2
65535 | chr5711a        | Nightlife Dress 2 Alt Color 1
65535 | chr5711b        | Nightlife Dress 2 Alt Color 2
65535 | chr5711_c01     | Nightlife Dress 2 (Quest Vernier)
65535 | chr5711_c02     | Nightlife Dress 2 (Quest Cervi)
65535 | chr5715         | Pole Dancer 1
65535 | chr5715a        | Pole Dancer 1 Alt Color 1
65535 | chr5715_c01     | Pole Dancer 2
65535 | chr5715_c01a    | Pole Dancer 2 Alt Color 1
65535 | chr5720         | Young Woman White Clothes 1
65535 | chr5720_c01     | Young Woman White Clothes 2
65535 | chr5720_c01a    | Young Woman White Clothes 2 Alt Color 1
65535 | chr5725         | Middle-Aged Woman White Clothes 1
65535 | chr5730         | Young Woman Coat 1
65535 | chr5730_c01     | Young Woman Coat 1 (No Hat) Alt Color 1
65535 | chr5735         | Middle-Aged Woman Coat 1
65535 | chr5735_c01     | Middle-Aged Woman Coat 1 (No Hat) Alt Color 1
65535 | chr5740         | Young Woman Swimsuit 1
65535 | chr5740a        | Young Woman Swimsuit 1 Alt Color 1
65535 | chr5741         | Young Woman Swimsuit 2
65535 | chr5741a        | Young Woman Swimsuit 2 Alt Color 1
65535 | chr5745         | Young Woman Swimsuit 1
65535 | chr5746         | Middle-Aged Woman Bath Getup 1
65535 | chr5750         | Young Woman Bracer 1
65535 | chr5751         | Young Woman Bracer 2
65535 | chr5753         | Young Woman Bracer 3
65535 | chr5760         | Young Woman Samue 1
65535 | chr5760_c01     | Young Woman Samue 2
65535 | chr5761         | Young Woman Samue 1
65535 | chr5770         | Hostess 1
65535 | chr5771         | Proprietress 1
65535 | chr5782         | Middle-Aged Woman (Eastern Theater Troupe) 1
65535 | chr5782_c01     | Middle-Aged Woman (Eastern Theater Troupe) 2
65535 | chr5790         | Idol 1
65535 | chr5790_c01     | Idol 1 (Personal Outfit)
65535 | chr5791         | Idol 2
65535 | chr5791_c01     | Idol 2 (Personal Outfit)
65535 | chr5795         | Young Woman Engineer 1
65535 | chr5795_c01     | Young Woman Engineer 2
65535 | chr5795_c02     | Young Woman Engineer 3
65535 | chr5800         | Pit Girl 1 (ETWS/Leslie)
65535 | chr5801         | Pit Girl 2 (Red/Linmei)
65535 | chr5802         | Pit Girl 3 (Leno)
65535 | chr5803         | Pit Girl 4 (Ingert)
65535 | chr5805         | Gekka School Martial Artist (Young Woman)
65535 | chr5810         | Academy Festival Dancer (Central Eastern Outfit Caucasian Dancer)
65535 | chr5811         | Academy Festival Dancer (Western Outfit Dancer)
65535 | chr5812         | Academy Festival Dancer (Eastern Outfit Dancer)
65535 | chr5813         | Academy Festival Dancer (Central Eastern Outfit Dancer)
65535 | etc0011         | Black Cat
65535 | etc0012         | Kitten
65535 | etc0013         | White Cat
65535 | etc0014         | Tabby Cat
65535 | etc0015         | Tabby Cat (Fat)
65535 | etc0020         | Camel
65535 | etc0020_c01     | Camel (Carrying Luggage)
65535 | chr5112         | NPC Fie 0
65535 | chr5112         | Fie
65535 | chr5112         | NPC Fie 2
65535 | chr0006         | Bergard Test
65535 | chr9991         | Male Test 1
65535 | chr9991a        | Male Test 2
65535 | chr9991b        | Male Test 3
65535 | chr0113         | Walter Test
65535 | chr0113_c01     | Walter Test
65535 | chr5112         | Fie Test
65535 | chr5116_c00     | Olympia Test
65535 | chr5300         | Naje Test
65535 | chr5316         | Ashen Lu Test
65535 | chr0327         | Edmond Auclair Test
65535 | chr5301         | Aida Test
65535 | chr0329         | Colonel Gerald Test
65535 | chr5317         | Saara
65535 | chr5318         | Shaheena
65535 | chr5320         | Kaela McMillan
65535 | chr5321         | Professor Latoya Hamilton
65535 | chr5322         | Kilika
65535 | chrx010         | Transparent Human
65535 | chr5660a_c01    | Republic Police Officer 1 (Young Woman; Central Eastern; Dirty Clothes Version)
65535 | etc0032         | Fox (Complete Version)
65535 | etc0040         | Dove
65535 | etc0041         | Dove 2
65535 | etc0050         | Crow
65535 | etc0310         | Stuffed Bear
65535 | mon0006         | Ball Bat
65535 | mon0012         | Military Dog Monster
65535 | mon0016_c00     | Armor Cruncher
65535 | mon0027         | Garudimimus
65535 | mon0030         | Pul Plantar
65535 | mon0034         | Abyss Worm
65535 | mon2001         | Hundred Eye Drome
65535 | mon2000         | Gargoyle
65535 | mon2002         | Zombie Aida
65535 | mon2003         | The Tyrant
65535 | mon2004         | Plant Hydra
65535 | mon2006         | Beast Naje
65535 | mon2012         | Ordro Nue
65535 | mon2018         | Vagrants Diaspora
65535 | mon0145         | Torso Smile
65535 | etc1000         | Longlai Futon
65535 | etc1001         | Oración Futon

## Файлы моделей без записи в t_name.tbl

Отобраны по имени файла.

- chr0000_c60
- chr0002_c60
- chr5003_c58
- chr5003_c60
- chr5117
- chr8500
- chr8997
- chr8998
- chr8999
