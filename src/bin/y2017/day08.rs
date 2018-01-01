use std::collections::HashMap;
use std::str::{self, FromStr};
use std::cmp;

pub fn part1(s: &str) -> i64 {
    let mut map = HashMap::<&str, i64>::new();
    for line in s.trim().lines() {
        let (reg, delta, condition_reg, condition) = parse_line(line);
        let conditon_reg_value = *map.entry(condition_reg).or_insert(0);
        let reg_entry = map.entry(reg).or_insert(0);
        if condition.eval(conditon_reg_value) {
            let value = *reg_entry;
            *reg_entry = value + delta;
        }
    }
    let max = *map.values().max().unwrap();
    max
}

pub fn part2(s: &str) -> i64 {
    let mut max = 0;
    let mut map = HashMap::<&str, i64>::new();
    for line in s.trim().lines() {
        let (reg, delta, condition_reg, condition) = parse_line(line);
        let conditon_reg_value = *map.entry(condition_reg).or_insert(0);
        let reg_entry = map.entry(reg).or_insert(0);
        if condition.eval(conditon_reg_value) {
            let value = *reg_entry;
            max = cmp::max(value, max);
            *reg_entry = value + delta;
        }
    }
    max
}

fn parse<T: FromStr>(v: &str) -> T
    where T::Err: ::std::fmt::Debug,
{
    match v.parse() {
        Ok(v) => v,
        Err(err) => panic!("unable to parse {:?}: {:?}", v, err),
    }
}

fn parse_line(line: &str) -> (&str, i64, &str, Condition) {
    let mut elements = line.split(' ');
    let register = elements.next().unwrap();
    let sign = if elements.next().unwrap() == "inc" { 1 } else { -1 };
    let num = sign * parse::<i64>(elements.next().unwrap());
    let _ = elements.next(); // if
    let condition_reg = elements.next().unwrap();
    let op = match elements.next().unwrap() {
        "==" => Op::Equal,
        "!=" => Op::NotEqual,
        ">" => Op::GreaterThan,
        ">=" => Op::GreaterThanEqual,
        "<" => Op::LessThan,
        "<=" => Op::LessThanEqual,
        s => panic!("Unexpected bytes: {:?} ({})", s, line),
    };
    let expected_value = parse::<i64>(elements.next().unwrap());
    (register, num, condition_reg, Condition { op, value:  expected_value })
}

#[derive(Copy, Clone, Debug)]
enum Op {
    NotEqual,
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(Copy, Clone, Debug)]
struct Condition {
    op: Op,
    value: i64,
}

impl Condition {
    fn eval(self, value: i64) -> bool {
        match self.op {
            Op::Equal => value == self.value,
            Op::NotEqual => value != self.value,
            Op::GreaterThan => value > self.value,
            Op::GreaterThanEqual => value >= self.value,
            Op::LessThan => value < self.value,
            Op::LessThanEqual => value <= self.value,
        }
    }
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 4416);
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 1);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 5199);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE), 10);
}

#[cfg(test)]
static EXAMPLE: &str = "
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

pub static INPUT: &str = "
d dec 683 if qn == 0
d dec -220 if h == 0
rak dec -875 if rak > -9
isy dec 250 if wf == 0
cie dec 20 if rak > 870
isy inc 93 if wf >= -5
o dec 739 if bok < 8
rak inc -605 if mxg <= 9
rak inc 668 if rfw > -8
rfw dec 214 if h > -7
j dec 649 if wf != 4
bok dec -712 if cie >= -22
s dec 151 if rxb == 0
bok dec -656 if d <= -463
pf dec -435 if brr != -10
pf dec 115 if rxb < 6
uxr dec -574 if brr == -3
h inc -34 if s == -151
rxb inc -919 if rak == 938
s inc 627 if o <= -748
rxb dec -456 if rfw != -214
rak dec -687 if x <= 8
d dec 292 if bok >= 1363
mxg inc 665 if o == -739
brr dec 531 if bok == 1368
isy dec -890 if x != 7
o inc -649 if bok > 1361
erb dec 656 if j != -648
vso inc -882 if wf <= -2
rxb inc 978 if brr >= -539
pf dec -176 if wf <= 8
rxb dec -647 if hsn == 0
vso inc -2 if isy <= 725
brr dec -661 if x != 4
uxr dec 913 if x <= 9
hsn inc -784 if rxb <= 706
pf dec -795 if hsn <= -782
pf dec 421 if rfw >= -217
o inc -88 if s < -147
rak inc 991 if hsn == -784
brr dec -27 if bok != 1372
uxr dec 705 if x >= -2
tss inc 130 if h != -35
vso inc -800 if wf != 8
tss inc 301 if pf != 866
ie inc -311 if cie == -20
mxg inc -20 if vso >= -808
cie dec -142 if bok == 1368
rfw dec -141 if rxb != 706
isy dec -826 if j > -656
cie inc 252 if pf >= 870
hsn dec 972 if bs < 2
hsn inc -576 if rfw >= -219
bok dec 439 if h >= -40
cie dec 614 if h == -34
cie dec 890 if cie == -240
wf inc -251 if pf <= 875
isy inc -507 if o == -1476
bok dec 619 if bok < 928
rxb inc 361 if rfw == -224
rfw dec -654 if s < -141
mxg dec -488 if isy >= 1047
pf dec 721 if mxg >= 1135
h inc -4 if rxb == 706
hsn dec -966 if rfw >= 431
isy dec -422 if h < -44
rfw inc -379 if qn == 0
pf inc -78 if cie >= -1136
o inc 902 if isy > 1042
erb dec -529 if brr <= 160
x dec 280 if o <= -572
isy dec -266 if d < -751
j dec -260 if isy != 1313
rak dec 330 if qn != 9
tss inc -113 if ie <= -311
isy dec -186 if erb > -135
o dec 839 if pf <= 800
x inc -906 if uxr > -1622
hsn dec 353 if vso > -808
cie dec -734 if bs > -9
rak dec -690 if rfw == 61
tss inc 355 if rfw == 54
isy dec 507 if erb != -127
uxr dec -901 if hsn >= -1726
tss inc 787 if cie == -396
j inc -851 if d < -753
d dec -929 if pf < 790
h inc -328 if mxg <= 1140
j dec -381 if erb != -122
o dec 810 if rxb <= 696
brr dec 207 if x == -1186
rak dec -697 if erb < -135
ie dec 96 if vso <= -798
vso inc 739 if rak < 2968
o inc 890 if mxg == 1133
wf dec 453 if pf <= 785
rxb inc -950 if bok > 923
brr inc 160 if isy != 1504
isy inc 767 if rak < 2967
bok dec 297 if bs != 2
qn dec -686 if erb == -127
uxr dec -835 if h >= -370
rxb dec -315 if pf < 799
h inc -97 if j != -864
bs inc -380 if erb < -117
rfw dec -444 if tss <= 1108
brr inc 951 if wf <= -251
erb dec -665 if h != -465
qn inc -350 if isy != 1504
h inc 603 if bs < -373
vso inc -545 if uxr != 111
o inc 110 if brr > 894
s dec -924 if rfw <= 509
hsn dec -114 if rfw != 507
tss dec -441 if erb != 529
d inc -791 if bok > 630
rxb dec 576 if pf >= 800
x dec 797 if j >= -859
cie inc 828 if rfw <= 507
s inc 945 if mxg != 1127
mxg inc 122 if qn <= 685
rak inc 778 if pf <= 786
tss inc -795 if pf < 794
vso dec -412 if d >= -1553
vso inc 371 if bs != -385
wf inc 443 if uxr != 109
wf dec 366 if rak == 2976
tss inc 608 if vso >= -571
x inc 316 if qn != 694
o inc -570 if mxg > 1136
qn inc 346 if bok < 636
erb dec 947 if erb != 529
uxr dec -673 if h != 147
pf inc -252 if d == -1546
ie dec -157 if o < -406
vso inc -46 if rfw > 499
qn dec -915 if vso <= -617
bok dec 288 if rak < 2983
o inc -842 if uxr == 783
rfw dec 867 if mxg > 1136
s dec -639 if uxr == 791
uxr dec 825 if erb < -406
rxb inc 414 if j >= -866
d dec 695 if vso > -612
mxg dec 119 if uxr == -34
tss inc -168 if bs > -379
tss inc 379 if mxg <= 1017
mxg dec 629 if ie != -250
rak inc 803 if d != -2245
pf dec -393 if rxb > 488
ie dec 574 if tss <= 1740
bok dec 498 if brr <= 907
rxb inc -999 if x < -1663
h inc -716 if s != 2359
vso inc -635 if cie > 430
tss inc 712 if mxg > 1009
x dec 303 if mxg == 1016
x dec -426 if rfw <= 510
rxb dec -99 if wf <= -174
ie dec 955 if x == -1241
ie inc -6 if qn < 1036
o inc 231 if pf != 541
d inc -967 if bok < -147
bs dec -86 if qn <= 1037
rak dec 255 if x != -1240
rfw dec 143 if d >= -3204
qn dec 175 if hsn > -1614
j inc -550 if isy == 1504
cie dec -553 if d != -3209
wf dec 353 if hsn == -1605
vso dec -563 if d <= -3205
o inc 593 if wf == -535
vso dec -519 if vso != -677
rfw inc 122 if s >= 2357
wf inc -889 if vso <= -166
ie dec -73 if rak >= 3531
pf dec 79 if d <= -3201
hsn dec 164 if cie > 982
rak dec 237 if rxb != -419
qn dec -585 if tss > 2442
rfw dec -186 if rfw >= 623
uxr dec 520 if rfw < 822
x dec 447 if x > -1251
vso inc -796 if pf < 464
tss dec 34 if rxb == -415
x inc 603 if cie != 994
uxr dec -234 if h <= -577
rak inc 381 if h < -571
j inc 692 if pf >= 455
pf inc -279 if pf != 461
bs inc -401 if h == -576
o inc -629 if mxg != 1018
hsn dec -619 if x < -1086
bok dec -91 if qn >= 1433
bs dec 660 if isy < 1495
x dec -908 if vso != -960
rak inc -16 if x >= -185
mxg dec -367 if mxg == 1010
bs inc 487 if o > -802
mxg inc -608 if o == -811
brr inc -730 if d < -3198
mxg dec -872 if qn != 1438
bs inc 467 if cie >= 983
isy inc 504 if cie < 981
rxb inc 356 if isy >= 1501
pf inc 427 if mxg < 1279
brr inc 967 if isy < 1510
rxb dec 561 if j > -720
hsn dec -175 if d != -3218
wf dec 700 if hsn >= -1602
h inc 637 if brr >= 1137
uxr inc 964 if wf >= -1232
rxb inc -739 if uxr > 401
erb dec -494 if qn == 1450
rxb inc -226 if qn != 1436
brr inc 131 if bs <= -220
rak inc -899 if mxg == 1274
tss dec -737 if bs != -238
j dec 700 if wf == -1220
j dec -823 if rak > 3654
j inc 466 if d > -3211
ie dec -521 if o == -811
d inc -898 if qn > 1440
isy dec 197 if rfw <= 815
erb dec -911 if vso > -952
j inc -835 if cie >= 983
erb dec -315 if uxr != 416
brr inc -673 if erb <= -99
erb inc -72 if vso <= -949
vso inc 165 if mxg <= 1285
qn dec -870 if vso >= -799
pf dec -574 if j == -1086
pf inc -858 if erb > -173
erb inc -243 if cie >= 985
rxb inc -927 if x == -177
rxb inc -260 if vso < -792
tss inc 62 if rak > 3647
tss dec -675 if vso <= -792
qn inc 807 if rfw <= 810
bok inc -360 if d > -4101
s dec 56 if brr != 1260
wf inc 855 if j == -1096
j inc 698 if s <= 2304
isy dec -459 if hsn <= -1591
s dec -577 if bok > -71
wf dec -469 if j != -397
pf dec 127 if tss != 3897
o inc 442 if rak >= 3651
d dec 941 if j <= -383
brr dec 809 if rak <= 3652
o inc 161 if rfw != 820
h dec 284 if uxr <= 418
j inc -805 if pf < 475
uxr inc 731 if uxr > 409
isy dec -589 if isy <= 1772
s inc 358 if h <= -221
wf inc 680 if rxb == -2519
brr dec 919 if ie != -1270
tss dec -77 if h > -226
qn dec 953 if hsn == -1594
s inc -553 if rfw == 813
isy dec 389 if qn < 1367
qn dec 676 if vso != -782
mxg dec 328 if uxr > 1135
wf dec 788 if hsn < -1599
uxr inc -361 if brr == -459
isy dec -775 if rxb > -2504
uxr inc 680 if vso < -788
tss dec 368 if tss <= 3967
pf inc -494 if qn <= 681
tss inc 84 if bok == -63
rfw dec -146 if rfw >= 804
erb dec -175 if pf >= 484
pf dec -655 if hsn <= -1590
j dec 273 if pf != 1133
rak inc -305 if wf > -764
o inc -234 if rxb != -2503
rfw dec -499 if uxr < 1470
bs inc -102 if qn < 687
j dec 698 if o > -435
cie inc -460 if cie > 977
wf inc 134 if rfw <= 1463
uxr dec -390 if h == -223
uxr dec -834 if bok >= -64
ie inc 538 if s == 2688
cie inc 504 if o > -445
bok inc 102 if ie < -1255
pf dec -65 if erb != -408
vso dec -317 if vso >= -791
x dec -139 if brr >= -459
s inc 772 if bok < 34
bs dec 736 if j <= -664
j dec -352 if vso != -794
s dec -656 if cie == 1029
rfw inc -919 if x <= -44
o inc 101 if vso < -793
bok dec 37 if bok == 39
hsn dec 413 if erb > -414
d inc 686 if rak != 3337
d dec -164 if j < -300
brr dec 968 if bok >= 1
rxb dec 177 if cie != 1029
j inc 200 if o >= -448
brr inc -665 if vso <= -792
vso inc 581 if rak != 3345
vso inc 491 if wf != -621
rxb inc 611 if rxb >= -2513
bok inc -77 if x != -39
cie dec 354 if qn <= 691
ie dec -328 if uxr != 2693
qn inc 629 if cie <= 679
mxg dec -941 if x > -46
tss inc -153 if x >= -46
o inc -459 if h == -222
ie inc -744 if s >= 3333
tss inc 62 if brr != -2086
tss inc -27 if bs == -330
vso dec -435 if rak != 3341
isy dec -751 if x != -38
hsn inc 700 if mxg != 1898
hsn inc -383 if wf == -624
rfw dec -697 if rfw >= 1458
brr inc -446 if ie > -1682
d dec 43 if rfw < 2152
pf dec -798 if rxb == -1909
vso inc -517 if x != -38
rxb dec -595 if rxb < -1892
s dec -820 if mxg <= 1884
o dec -502 if uxr >= 2686
wf inc -541 if rak < 3340
d inc 285 if tss != 3572
vso dec -901 if brr != -2543
uxr dec 402 if rxb != -1306
erb dec -927 if erb > -416
tss dec -62 if rxb != -1312
o inc -194 if qn >= 1319
hsn inc 734 if isy <= 1972
tss dec 553 if qn > 1308
d inc -33 if vso == 1616
bok dec 775 if tss <= 3080
hsn dec 119 if cie >= 669
x inc -50 if j <= -106
erb dec -422 if qn >= 1309
bs dec -573 if rxb <= -1303
uxr dec 431 if uxr <= 2692
rxb inc 174 if uxr <= 2255
tss inc -613 if wf >= -627
brr inc 166 if d != -3936
rak inc -947 if brr <= -2365
bs dec -504 if qn == 1312
cie inc 259 if wf > -630
hsn inc -290 if hsn == -1075
isy dec -778 if mxg <= 1899
rxb inc -146 if x >= -90
bs inc -812 if cie > 929
cie dec -71 if d <= -3943
vso dec 235 if d >= -3935
qn inc 192 if rak >= 2391
rfw inc 403 if erb == 940
rak inc -881 if rxb >= -1285
x dec 30 if rak > 1510
s inc 541 if rxb < -1268
hsn inc -622 if tss != 2453
rak inc -778 if erb <= 946
hsn dec 300 if wf <= -628
hsn inc -72 if ie != -1688
erb inc -145 if erb > 936
x inc -99 if o < -440
vso inc 866 if bok > -842
vso dec -308 if uxr <= 2256
rfw dec 487 if uxr != 2260
rfw dec -21 if mxg > 1881
h inc 866 if rxb <= -1273
erb dec 234 if d >= -3951
erb dec -658 if qn == 1504
o dec 771 if rfw == 2092
h inc -843 if brr < -2377
ie inc -528 if bok >= -851
qn dec 187 if rfw == 2086
isy inc -308 if d <= -3946
rfw inc -163 if vso < 1930
isy inc 815 if h == 643
mxg dec 606 if h < 642
mxg dec 165 if vso > 1923
rfw dec -530 if uxr <= 2253
ie inc 129 if mxg == 1726
uxr dec -505 if h != 638
bok inc -535 if bok >= -851
o dec 941 if j != -104
rak dec -133 if rfw >= 2460
bs inc -461 if mxg >= 1727
j dec -938 if brr != -2368
erb inc 737 if x >= -207
tss dec -280 if hsn < -2051
s dec 651 if bs == -65
rak inc -979 if vso < 1934
pf dec -487 if hsn != -2060
qn inc 384 if j != 830
j inc -389 if o < -2147
hsn dec 14 if s > 3224
rxb dec -710 if uxr <= 2766
hsn inc 278 if s != 3229
tss inc 376 if brr < -2370
d inc 306 if j != 435
wf inc -561 if tss > 3122
h dec -154 if bs >= -65
x inc 396 if hsn != -2073
mxg dec -806 if cie < 1013
hsn dec 941 if hsn <= -2078
erb inc 18 if pf > 1677
x inc 63 if rak > -244
rfw inc 883 if tss == 3124
s inc -615 if erb < 1241
s inc -696 if vso < 1933
qn dec 120 if pf <= 1678
rxb dec -680 if rxb <= -563
wf inc -880 if ie > -2087
tss dec 422 if d <= -3641
d dec 270 if cie > 1004
h dec 993 if tss > 3114
isy dec 490 if o != -2144
bok dec -649 if h > -200
bs dec -505 if o != -2164
hsn inc -863 if rfw == 2459
bok inc -307 if bs <= 442
x dec 961 if rxb > 104
pf inc -606 if brr >= -2376
mxg dec 409 if qn >= 1880
rxb inc -962 if ie == -2079
bs dec 847 if hsn >= -2944
hsn inc 543 if erb == 1237
d dec -752 if ie != -2086
h inc 90 if rak != -228
j dec 620 if cie < 1003
o inc 827 if o != -2156
erb inc -601 if bs >= -408
mxg inc -616 if o >= -1335
h dec -583 if rfw < 2468
rfw inc -727 if erb <= 643
s dec 178 if rak < -237
rfw inc -781 if x >= -1122
hsn inc 503 if brr < -2365
h dec 738 if hsn != -1888
vso inc 10 if rfw != 961
tss dec 455 if j != 430
pf dec -654 if bok >= -1047
mxg dec -953 if hsn == -1890
rfw inc -859 if cie <= 1008
rxb inc -957 if x >= -1112
erb inc 775 if isy >= 3061
rfw inc -574 if rxb == -850
erb inc 304 if s <= 1746
bok dec 703 if bs <= -413
d dec 788 if bs <= -404
uxr inc -714 if bok > -1035
bs dec -244 if h <= -264
h dec -530 if vso == 1934
s inc -792 if bs != -407
j inc 384 if rfw >= -484
rfw dec -694 if cie <= 1013
hsn inc -53 if uxr != 2762
rxb dec 83 if j != 834
rxb inc 849 if o >= -1329
o dec -244 if j < 816
x inc 812 if rxb <= -78
pf dec -950 if brr == -2372
x inc -169 if hsn == -1943
mxg inc -673 if vso != 1929
tss dec -932 if pf == 2682
o dec 976 if pf <= 2685
mxg inc -90 if mxg >= 1783
pf inc -466 if pf <= 2690
rxb dec -592 if uxr < 2760
s dec 904 if pf <= 2216
o inc -958 if qn != 1888
d inc 317 if ie != -2077
d inc 732 if wf > -1511
tss dec -200 if tss <= 3584
mxg inc -894 if mxg >= 1692
uxr dec -883 if rak > -248
d dec 576 if x >= -481
mxg inc -418 if d <= -3478
rxb dec -435 if tss == 3594
rxb dec -200 if uxr != 3646
j dec 70 if d != -3473
cie inc 524 if d > -3469
tss dec 988 if rak <= -233
bok inc 567 if pf < 2226
tss inc -486 if isy > 3064
erb inc -952 if brr < -2380
uxr inc 723 if rfw <= 214
x dec -510 if ie != -2078
d inc 395 if rfw <= 212
erb inc 592 if isy > 3075
ie inc -61 if d >= -3080
rak inc 400 if qn >= 1898
erb dec 449 if cie > 997
ie inc -918 if pf > 2215
s dec -711 if rfw <= 220
rxb dec 519 if wf != -1504
ie dec -59 if rxb < 1145
erb dec 425 if tss == 2120
cie inc -306 if isy <= 3069
x dec 635 if o >= -2296
wf dec 815 if tss == 2120
d dec -366 if pf >= 2210
cie inc -787 if o < -2302
erb dec -305 if brr != -2374
rfw dec -915 if pf > 2216
bs inc -516 if hsn >= -1947
rxb inc 845 if tss != 2120
erb dec -201 if qn == 1888
h inc 297 if tss < 2129
h dec -860 if s > 1541
isy dec -59 if hsn > -1949
mxg inc -266 if wf > -2316
vso dec -578 if d != -2703
qn dec 420 if o <= -2299
ie dec 307 if wf == -2319
pf dec -948 if bs >= -929
j dec 927 if h == 1416
vso dec 879 if d > -2717
j dec -543 if uxr != 4374
d dec -20 if o != -2308
brr inc 606 if erb > 1340
hsn dec 547 if rak != -244
cie dec -865 if uxr < 4366
bs inc -408 if o <= -2294
wf inc 938 if rfw < 207
pf inc 633 if j != 1289
rak dec -871 if d <= -2687
hsn inc 311 if tss < 2125
wf dec -231 if qn <= 1477
mxg dec 617 if rxb < 1137
hsn inc 20 if ie > -3315
uxr inc -810 if hsn < -2154
d dec -738 if mxg > 798
o inc 513 if d < -1960
s dec 953 if mxg <= 812
s inc 664 if cie < 775
h dec 18 if bok >= -480
erb dec 43 if rak > 631
vso inc 662 if rak > 632
mxg inc -938 if wf >= -2083
brr inc -359 if rxb > 1136
s dec -402 if rxb < 1144
h dec 59 if x < 48
mxg inc 842 if bs == -1331
ie inc -748 if vso >= 2298
mxg inc -176 if erb > 1301
s inc -345 if bs == -1335
cie inc 52 if brr < -2125
j inc -243 if wf > -2095
d dec 992 if qn != 1468
h inc 302 if tss == 2120
rfw dec 431 if hsn >= -2168
hsn inc -930 if wf > -2098
x dec -477 if cie != 767
erb dec 922 if pf <= 3797
j dec 838 if cie <= 782
mxg inc -96 if bs < -1324
cie dec 707 if hsn > -3099
mxg inc 258 if h < 1655
hsn dec -441 if qn >= 1461
j dec 518 if tss >= 2115
tss dec -760 if ie == -3306
ie dec 21 if d != -1948
j dec -990 if s > 997
bs inc -889 if bok >= -482
s dec 732 if hsn == -2646
pf dec -401 if hsn <= -2639
rak inc 211 if isy != 3128
brr inc -868 if uxr == 3554
vso dec 811 if uxr <= 3563
pf dec 232 if rxb >= 1137
bok inc -193 if vso < 1490
d dec 817 if qn == 1468
rak inc 282 if bs < -2216
o inc 74 if pf < 3972
isy dec -602 if rxb <= 1148
pf inc -6 if cie <= 69
uxr inc 835 if rfw >= -228
tss inc 790 if bok <= -661
rak dec -719 if bok <= -662
cie inc 978 if j > -298
d dec 475 if brr > -2994
rxb dec -348 if pf >= 3976
o dec 62 if j < -301
brr dec 486 if bok > -675
isy dec -256 if hsn == -2648
rxb dec -536 if tss < 3671
ie dec 91 if vso < 1490
h inc 53 if h >= 1644
qn inc 621 if d > -3247
hsn dec -433 if x <= 523
x dec 703 if tss >= 3673
x inc -557 if isy >= 3978
bs inc -180 if h < 1709
pf inc -25 if hsn <= -2207
bs dec 761 if o > -2294
uxr dec 989 if brr > -3482
erb dec 385 if h > 1703
hsn dec 339 if bok < -664
isy inc -176 if s > 989
hsn inc 405 if isy == 3810
pf dec -849 if uxr < 3402
bok inc 274 if rak > 1631
j inc 433 if tss >= 3668
ie dec 385 if o <= -2290
rak inc 529 if tss >= 3670
isy dec -467 if o < -2285
erb inc 765 if erb >= 7
vso dec -935 if wf >= -2084
tss inc 21 if brr != -3487
s dec 631 if erb > 5
hsn inc 709 if h < 1709
isy inc 754 if tss != 3691
rak dec -730 if rfw <= -216
erb inc 33 if bs <= -3156
hsn inc 972 if rxb <= 1681
d inc 902 if mxg <= 1634
cie inc -544 if d == -2343
wf dec -706 if bs <= -3160
j dec 564 if isy < 4282
ie inc -832 if o > -2298
rfw inc 737 if pf >= 4795
o inc 558 if o <= -2285
erb dec 749 if bs > -3163
pf dec -93 if h > 1700
o dec 85 if isy > 4271
brr inc -713 if x <= -45
x dec 659 if d < -2344
tss dec -316 if mxg < 1638
pf inc -542 if bok < -388
tss dec 572 if h <= 1708
s inc -19 if cie != -464
j dec -262 if hsn <= -463
rfw dec -775 if rxb > 1677
s inc 564 if ie > -4639
o inc -484 if brr == -3479
uxr dec -651 if hsn <= -463
wf dec 866 if qn == 2094
ie inc 635 if pf < 4339
cie inc 248 if pf == 4341
rxb dec 233 if o > -2299
bok inc 914 if cie != -232
hsn dec -748 if x == -47
cie dec 322 if uxr <= 4046
mxg inc 79 if brr <= -3477
tss dec 939 if x == -42
pf dec 773 if s < 1551
bok inc -145 if mxg >= 1712
uxr inc -237 if tss > 2494
d dec 187 if mxg >= 1705
ie inc -280 if o > -2305
rak inc -757 if isy > 4273
s dec -922 if isy != 4278
pf dec 868 if qn <= 2089
pf dec 838 if j >= -174
ie inc 293 if rfw > 554
bok dec -846 if o >= -2303
o dec -869 if qn == 2089
rak inc 960 if cie <= -224
vso dec -192 if o != -1443
o dec -477 if rxb != 1675
bs dec 256 if uxr > 3813
brr dec 224 if bok > 1360
qn dec -294 if erb <= -726
h dec 790 if ie >= -4628
erb inc -964 if rak != 3104
rfw dec 417 if s >= 2462
wf inc -396 if pf == 1862
wf dec 765 if s > 2457
bs inc -213 if mxg < 1711
rfw dec 18 if o > -963
isy dec 99 if uxr != 3808
uxr dec -548 if mxg > 1707
hsn dec 404 if brr == -3703
cie dec -420 if x <= -49
erb dec -623 if rxb < 1682
h dec 519 if rxb > 1682
o dec -58 if rak < 3106
vso inc 459 if rxb == 1679
hsn dec 843 if rfw < 126
mxg inc 820 if pf < 1864
hsn inc 60 if ie >= -4629
brr dec 25 if rxb == 1679
uxr dec 148 if wf > -2545
s dec 416 if o == -897
hsn inc -427 if ie != -4625
s inc -925 if erb >= -1066
tss dec -413 if d != -2530
j dec 346 if tss <= 2500
uxr inc -160 if wf != -2542
wf inc -718 if bs == -3630
j inc -518 if brr != -3728
hsn dec -489 if bok <= 1372
bok dec -28 if tss == 2496
bok dec -106 if x != -34
d dec 905 if erb == -1060
bok dec -303 if s <= 1529
vso inc 650 if uxr > 4053
cie inc -209 if ie < -4627
erb dec 272 if d <= -3435
h dec 100 if wf != -3253
ie inc -783 if d != -3425
wf dec -712 if vso == 2785
pf dec 133 if ie > -5403
brr dec 218 if bs < -3625
qn dec -420 if rfw < 127
bok dec -633 if s < 1546
cie inc -932 if ie > -5412
wf dec 515 if pf > 1857
mxg inc 454 if ie <= -5411
x inc -77 if hsn == -1593
o inc 746 if brr <= -3950
x dec 872 if erb >= -1332
vso inc 268 if h != 815
h dec -436 if bs != -3624
erb dec -543 if bok < 2141
ie dec -375 if tss == 2496
x dec -176 if rxb > 1672
pf dec 926 if rfw <= 124
bs inc -134 if pf <= 938
isy dec 664 if rxb != 1684
s inc 584 if isy >= 3523
vso dec 913 if rak >= 3098
bs inc 745 if wf == -3064
rfw inc 309 if qn == 2515
uxr dec -496 if isy <= 3515
hsn dec -633 if isy <= 3514
brr dec 502 if j <= -521
brr inc -521 if j != -519
tss dec 302 if o != -891
hsn inc 340 if ie != -5030
cie dec 959 if j > -522
wf inc 417 if erb <= -781
isy dec -376 if d <= -3444
h inc -131 if cie != -2112
bs inc -510 if vso < 3055
x inc -780 if rxb > 1676
ie dec 702 if tss >= 2187
rfw inc 590 if h <= 1124
wf dec -967 if hsn > -969
vso dec -104 if s > 1536
hsn inc 197 if brr <= -4468
qn dec -309 if ie == -5730
x inc 694 if brr > -4472
isy dec -704 if vso > 3164
j inc -127 if x >= -904
pf inc -579 if uxr != 4550
bok inc -666 if rak > 3092
vso dec -926 if hsn <= -969
brr dec 71 if brr > -4467
s dec 10 if o > -904
j inc -338 if x == -901
cie dec -630 if x >= -910
cie dec -716 if ie < -5737
wf inc 613 if wf <= -1678
s dec -409 if bs <= -3539
rfw inc -505 if d < -3432
bok inc 912 if bs == -3529
isy inc -992 if mxg > 2525
bok dec 100 if tss <= 2196
s inc 712 if s > 1520
cie dec -381 if brr != -4465
brr dec 793 if cie > -1099
bok dec -387 if bok < 2283
s dec 812 if mxg > 2520
hsn dec 422 if isy >= 2519
brr dec -597 if wf <= -1063
tss inc -539 if h == 1119
d dec 159 if hsn == -1382
h dec 474 if wf <= -1067
h dec -34 if vso < 3161
rxb inc 714 if pf != 936
wf dec -564 if pf > 926
d inc 534 if erb >= -781
h inc 808 if j == -982
brr inc 976 if qn == 2509
o dec 623 if rfw > 201
hsn dec -794 if h < 1492
uxr inc 649 if mxg <= 2533
cie inc 156 if rak == 3096
j dec -392 if hsn == -588
tss inc -161 if rfw != 213
rfw dec 864 if qn > 2512
d inc -499 if cie == -950
rxb inc -849 if rfw > 203
x inc -478 if pf == 939
j inc -343 if uxr > 5194
qn inc 504 if d < -4087
pf inc -430 if s > 1423
o inc -709 if x <= -911
mxg inc 594 if rxb != 824
h dec -885 if h == 1487
brr inc -663 if bok <= 2657
tss inc 504 if bs != -3532
brr inc -100 if s < 1432
bs inc 133 if rfw == 206
hsn inc 535 if rfw >= 200
hsn inc -82 if h != 2372
mxg dec 254 if rak != 3090
wf inc -401 if o <= -1513
d dec 577 if qn < 3019
qn inc 94 if bok > 2660
pf dec -990 if ie <= -5732
bok dec 181 if cie < -945
qn dec 431 if bok <= 2489
mxg inc 128 if qn >= 2673
d dec -233 if tss > 1996
rak dec 364 if ie != -5732
tss inc 174 if o == -1521
tss dec -463 if isy != 2530
rak inc -238 if bs >= -3403
brr dec 447 if j == -933
ie dec 853 if cie > -952
bs dec 350 if bs < -3393
brr dec -996 if d <= -4434
cie dec -749 if tss < 2634
mxg inc -484 if bok == 2484
vso dec -135 if x == -899
d inc -426 if rxb < 832
h dec -40 if uxr != 5199
rak inc -480 if brr == -2445
h inc 818 if s == 1428
hsn dec -708 if hsn < -51
rfw inc -155 if rxb != 838
pf inc -3 if o < -1511
qn dec -144 if ie <= -6579
hsn dec 166 if s > 1418
rfw inc 216 if bok == 2484
h inc 65 if isy <= 2523
uxr inc -405 if ie <= -6592
h inc -85 if wf <= -896
rfw dec 104 if s == 1428
ie dec -144 if vso != 3157
ie dec -877 if rfw < 156
uxr dec 836 if isy >= 2514
mxg inc -753 if uxr >= 4358
j inc -695 if x >= -901
rak dec -399 if wf >= -910
d dec 289 if vso != 3157
hsn inc 482 if j == -1628
rfw dec 10 if bs >= -3741
vso inc 803 if d != -4864
vso inc -335 if qn == 2820
o dec 527 if rak != 2776
tss dec 799 if rfw > 170
j dec -763 if hsn > 967
tss dec 259 if pf <= 1498
wf inc -260 if rxb <= 820
x inc -560 if hsn < 980
j inc 126 if wf <= -898
uxr inc -665 if j != -736
vso inc -59 if h >= 3170
j dec 88 if isy < 2528
brr inc 622 if uxr < 3692
wf dec -81 if isy >= 2521
rfw inc 497 if tss > 2370
d inc 411 if erb <= -789
cie inc -375 if mxg == 1761
bok inc -998 if hsn <= 972
pf inc 224 if mxg != 1757
x inc 551 if rxb <= 835
qn inc -388 if brr != -2442
rfw inc -79 if rak < 2779
hsn dec 756 if j > -831
wf dec -67 if ie > -6589
hsn dec -175 if tss < 2381
tss dec -861 if rak == 2777
o dec -461 if rak >= 2772
rxb dec 916 if erb <= -795
pf dec 379 if erb == -789
isy inc -155 if bok == 1486
erb dec 790 if o == -1587
brr inc 445 if tss < 3242
rxb inc 884 if hsn > 394
tss dec 591 if x == -913
j dec 298 if cie == -1325
ie dec -68 if mxg <= 1764
mxg dec 387 if vso < 3570
tss dec -86 if bs != -3748
rxb inc 238 if hsn >= 390
rak inc -28 if bok == 1486
rak inc -640 if mxg == 1374
qn dec -83 if tss >= 3314
tss dec -238 if bok != 1476
tss dec -101 if o != -1587
hsn dec -537 if tss == 3561
wf inc -859 if brr == -2000
tss inc -35 if uxr >= 3698
cie inc -970 if rak <= 2112
rfw inc 971 if h < 3172
wf dec 887 if rfw < 1555
bok inc 20 if o <= -1592
mxg inc -326 if cie > -2298
mxg inc -684 if j > -1116
brr inc 639 if qn >= 2510
rxb dec -332 if rxb < 1069
uxr dec 189 if brr > -1369
rak dec -567 if rfw == 1552
cie dec 407 if uxr > 3505
h dec -365 if o < -1585
cie dec -990 if s == 1428
rfw dec 368 if ie == -6517
vso dec -76 if wf != -2502
erb inc 334 if bs >= -3747
o inc 583 if qn == 2515
pf inc 7 if rxb > 1399
o inc 166 if qn >= 2512
wf inc 492 if x <= -905
uxr inc -566 if d == -4452
bok inc -919 if isy >= 2364
hsn inc 517 if rak != 2677
j dec -916 if d != -4454
bok dec 116 if rak > 2674
vso dec -477 if x < -907
qn dec -964 if erb == -1245
j dec 987 if qn > 3478
rak dec 317 if d < -4442
isy dec 1 if cie > -1717
j dec 95 if mxg == 1046
h dec -927 if s <= 1435
o dec 812 if ie >= -6518
brr inc -499 if uxr >= 2936
rak dec 716 if cie == -1712
x inc 482 if pf == 1350
cie inc -272 if mxg != 1049
isy inc -839 if brr == -1870
o inc -259 if erb <= -1239
j dec 859 if x != -910
isy inc 493 if tss != 3521
rxb dec -673 if mxg >= 1043
vso dec 35 if uxr <= 2947
rxb dec -128 if bok < 459
bok inc 645 if isy >= 2853
bok inc 384 if s < 1430
h inc -709 if brr != -1865
o inc 638 if cie != -1980
rak dec 9 if pf != 1345
erb dec -772 if rfw != 1176
ie inc -817 if rfw == 1184
vso dec 21 if ie == -7334
s inc -829 if uxr > 2936
isy inc -205 if cie <= -1984
mxg dec -404 if o == -1271
x inc 633 if mxg < 1462
vso inc 795 if bok <= 1481
cie inc 613 if h == 3753
s inc 156 if qn != 3478
wf inc 908 if erb > -467
x inc -340 if j < -1191
bok dec -790 if uxr > 2940
d inc 588 if vso >= 4774
ie dec 711 if rxb < 2209
j dec -687 if isy >= 2657
brr inc -979 if bok == 2270
uxr dec -130 if isy < 2649
h inc -403 if pf == 1341
pf dec -156 if ie == -8045
o dec 828 if bok <= 2276
bs inc -379 if d != -3864
vso inc 570 if isy != 2654
brr dec -208 if brr > -2837
d dec 364 if wf != -2005
pf inc -459 if d <= -4219
wf dec 449 if tss != 3536
s dec -477 if wf >= -2464
j dec 366 if vso == 4782
s dec 240 if rxb != 2201
hsn dec -892 if h > 3752
ie dec -551 if o <= -2098
wf inc -574 if tss < 3531
s dec -972 if isy >= 2664
isy dec -632 if pf <= 1042
uxr inc -733 if vso > 4779
bok inc -541 if o == -2099
pf inc -723 if cie >= -1378
brr inc 993 if isy != 3285
isy inc -859 if brr == -1851
qn inc -925 if rfw != 1184
brr dec 118 if j != -1572
rak inc -711 if hsn <= 2343
o dec 896 if rfw <= 1184
qn inc -332 if h <= 3759
rak dec -684 if mxg == 1452
qn dec 945 if uxr <= 2216
d inc 971 if rfw < 1189
bok dec -80 if o != -2995
hsn dec 288 if uxr >= 2219
x inc -986 if bok > 1725
o inc -640 if tss > 3527
rak dec -210 if isy == 3287
qn dec -937 if j > -1570
d dec -796 if ie >= -7497
rfw dec 869 if d != -2456
j dec 941 if erb == -473
vso inc -366 if j > -2509
h inc -526 if d != -2461
";
