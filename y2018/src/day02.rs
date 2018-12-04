use aoc_macro::{generator, solution};

#[generator]
fn generator(input: &str) -> &str {
    input.trim()
}

#[solution(part1,
    example_input = &["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"].join("\n"),
    example = 12,
    expect = 8398)]
fn part1<'a>(input: &str) -> usize {
    let mut twice = 0;
    let mut three = 0;
    for line in input.lines() {
        let mut map = fnv::FnvHashMap::default();
        for byte in line.as_bytes() {
            *map.entry(*byte).or_insert(0) += 1;
        }
        twice += map.values().any(|c| *c == 2) as usize;
        three += map.values().any(|c| *c == 3) as usize;
    }
    twice * three
}

static EXAMPLE_2: &str = "
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

#[solution(part2,
    example_input = generator(EXAMPLE_2),
    example = "fgij",
    expect = "hhvsdkatysmiqjxunezgwcdpr")]
fn part2<'a>(input: &str) -> String {
    for line_a in input.lines() {
        'line_b: for line_b in input.lines() {
            let it = line_a.as_bytes().iter().cloned().zip(line_b.as_bytes().iter().cloned());
            let mut difference = 0;
            for (a, b) in it {
                if a == b { continue; }
                difference += 1;
                if difference > 1 {
                    continue 'line_b;
                }
            }
            if difference == 1 {
                return line_a.chars().zip(line_b.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect::<String>();
            }
        }
    }
    unreachable!()
}

static INPUT: &str = "
ohvflkatysoimjxbunazgwcdpr
ohoflkctysmiqjxbufezgwcdpr
ohvflkatysciqwxfunezgwcdpr
fhvflyatysmiqjxbunazgwcdpr
ohvhlkatysmiqjxbunhzgwcdxr
ohvflbatykmiqjxbunezgscdpr
ohvflkatasaiqjxbbnezgwcdpr
ohvflkatyymiqjxrunetgwcdpr
ohvflkatbsmiqhxbunezgwcdpw
oheflkytysmiqjxbuntzgwcdpr
ohvflkatrsmiqjibunezgwcupr
ohvflkaiysmiqjxbunkzgwkdpr
ohvilkutysmiqjxbuoezgwcdpr
phvflkatysmkqjxbulezgwcdpr
ohvflkatnsmiqjxbznezgpcdpr
ohvylkatysriqjobunezgwcdpr
ohvflkatytmiqjxbunezrwcypr
ohvonkatysmiqjxbunezgwxdpr
ohvflkatgsmoqjxyunezgwcdpr
ohvflkbtqsmicjxbunezgwcdpr
ohvflkatysmgqjqbunezgwcdvr
ohvtlkatyrmiqjxbunezgwcdpi
ohvflkatyskovjxbunezgwcdpr
ohvflkayysmipjxbunezgwcdpu
ohvalkltysmiqjxbunezgecdpr
ohvflkatysmiqjxiunezgnndpr
ohvflkatyomiqjxbbnezgwcdpp
ohvflkatysmiqjxbuoezgncdpy
omvflkvtysmiqjxwunezgwcdpr
ohvflkatynmicjxbunezgwpdpr
ohvflkatyqmaqjxbunezvwcdpr
ohbfhkatysmiqjxbunezgwcdqr
ohvflkatesmiqjvbunezpwcdpr
ohvflkatysmsqjxiunezgwcdhr
ohvfjkatysmwqjxbunezgwcddr
ohvflkanysmiqjxbunwkgwcdpr
ohqflkatysmiqjxbuuezgwcddr
ohvflkatysmvqjxbznlzgwcdpr
ohvflkatysmiqjxbunjzwwqdpr
ohvfjkatysmiqxxbunezgwcupr
chvfxkatysmiqjxxunezgwcdpr
uhvflkatitmiqjxbunezgwcdpr
ohvflbatysmiqjxbuntzgwcdor
ohvflkmtysmmqjxbunexgwcdpr
ohvflsatysmyqjxjunezgwcdpr
ohvfskatysmiqjjbunezgwcdpg
ohvflkatysniqjxbunexgwcrpr
ohvfekatysmiqjxbunedswcdpr
ohvfltatysmjqjxbunezghcdpr
ohvflkatydmiqjxvunezggcdpr
oavflkatysmiqjxtunazgwcdpr
ohvflkltysmiqjxbuzeugwcdpr
ohbflkatysmiqjybuuezgwcdpr
ehvfzkatysmiqjxbuhezgwcdpr
odvflkatssmiqjxbunezgwcdpj
ohvflkatysmiqjzbufezgwbdpr
jhvflkdtysmiqqxbunezgwcdpr
ohvflkatysmiqjwbunengwcnpr
ohvfskatysmiqjxbxuezgwcdpr
ohvflkatysmiqjobvnezgwcrpr
ohvrlkatysmiqjxbwnezgrcdpr
ofvflkatysmiqjxbunezpwcdwr
ohvfxdatyomiqjxbunezgwcdpr
yhvflkatydmiqjxbubezgwcdpr
ohvflkatysdiqjxbuneztwcspr
ohvflkatydmiquxbunezgwcbpr
ohvflkatysmiqcxbukezgwcdwr
ohvflkntasmiqjxbunezghcdpr
lhvflkatysmiqjxbunezqwckpr
ehifikatysmiqjxbunezgwcdpr
ohvflkatysmiqjcbutezgwcdpm
ohvflkatjssiqrxbunezgwcdpr
oyvflkavysmiqjxlunezgwcdpr
orvflkgtysmiqjxbukezgwcdpr
ihvflkatysmiqaxbunpzgwcdpr
ohvflkatusmiqjxbbnezgwchpr
ohvflkatysbiqjxvuneugwcdpr
ohvflkatysmiqjcbungzgwcwpr
ovvflkatysmidjxbunezgscdpr
ohvflqatysmiljxbunfzgwcdpr
ghvfokatysmiqjxbunqzgwcdpr
nxvflkatysmxqjxbunezgwcdpr
ohvflkatysmiqjxbexezgwrdpr
ohvfrkatysmhqjxbuntzgwcdpr
ohvflkvtysmiqjxocnezgwcdpr
ohvglkgtysmiqjxnunezgwcdpr
ohvflkatysmnqjxbunecgwqdpr
oyvflkatysgiqjxbcnezgwcdpr
ofvflkatysmiqjxbunfzgwcdpg
otvflkttysmiqjxbunezgwmdpr
ohvflkvtysmiqjbbunezgzcdpr
ahvflkatysyiqjxbunezvwcdpr
ohiflkatysmydjxbunezgwcdpr
ohvfwkatysmvqjxbunezwwcdpr
ohvflkatysbiqjxbunergwodpr
hhvsdkatysmiqjxbunezgwcdpr
ihvflkwtysmiqjxbunezgacdpr
ohvfljatysmiqcxbunuzgwcdpr
ohvflkatysqiqlwbunezgwcdpr
ohvflkauysmkqjxwunezgwcdpr
ohvflkatysmoqjqbunezgwodpr
ohvslkvtysmipjxbunezgwcdpr
olvflkatysmiujxbunezgwctpr
osvflxatysmiqjxbenezgwcdpr
orvflkhtysmiqjxbinezgwcdpr
ohcflkatystiqjxbunezbwcdpr
ohcflkatyfmifjxbunezgwcdpr
ohvflkatdsmiqjxbrnezgwcdpt
ohvflkatysmiqjxbwnqzawcdpr
oevflkakysmiqjxbunezgwcdpt
ofvflkatysmiqjxbunbqgwcdpr
ohvflkatysmdqjxbunefqwcdpr
ohvklkalysmiqjxbunezgwcepr
ocvflhatysmiqjxbunezzwcdpr
uhvflkatysmiqmxbunezgwcxpr
ohvflkatyshikjhbunezgwcdpr
lbvflkatysmoqjxbunezgwcdpr
ohvflkatssmuqjxbunezgscdpr
ohvflkatysmifyxbuvezgwcdpr
ohvfikatysmiqjxbunezgwfupr
ohvmlkaiysmiqjxqunezgwcdpr
ohvflkatysmiqjxiunpzgwcdpo
lhvflkatysmpqjxbenezgwcdpr
ohvflkatysmiqjobunengwczpr
ohoflkatysniqjxbunezgccdpr
ohvfxkatysmiqjgbunyzgwcdpr
ohvflkytysmiljxbubezgwcdpr
hhvsdkatysmiqjxjunezgwcdpr
ohvflkatysmiqjtuunezgwcdpt
ohvfdkxtysmiqjubunezgwcdpr
ohxflkatysmiyjxbunezgwcdhr
ohvflkatysmiqjibunezgwcppd
ohvflkatysmihjxbunezgwcdhj
ohvflkatysmiqjxronezgwcdvr
ofrflxatysmiqjxbunezgwcdpr
ohvwlkatysmiqjxounezgscdpr
ohvflkatcodiqjxbunezgwcdpr
oqvflkatysmiqjxbunebgwmdpr
ohvflmatysmisjxbunezqwcdpr
ovvflkatysmiqjxbuxezgwcdpe
ohvflkatysmdejxbuneztwcdpr
hhvflkathsmiqjxbwnezgwcdpr
ohkflkatlsmsqjxbunezgwcdpr
ohvflkktysmizjxhunezgwcdpr
ohzflkatysmiqjrbunezgwcdpj
ohuflwatysmiqjxbunezgwcdgr
ohvflkatysmiqvxmunpzgwcdpr
xhvflkwtysmiqjxbunezgwjdpr
whvflkatysmiqjxbunezgzcopr
ohvflkayysmiqjxuznezgwcdpr
khvflkasysmiqjxbunezgwcdpv
ohvflkatylmiqjxbpnozgwcdpr
ohvflkgtysziqjxbunezgwgdpr
ohvfljaiysmiqjxbuvezgwcdpr
ohvflkxtyslizjxbunezgwcdpr
ohzflkatysmiqjxbcnezgwcdar
ohvflkatysmiqjxbisecgwcdpr
shvflkatyjmiqjkbunezgwcdpr
mhvflkatysmiqjxvunezgwcdpk
ohfflkatysmiqjxbunczgwcppr
ohvflkatysmiqjkzunezgwcdpc
ohvflkatysmifjxbuneygwctpr
ohvflkatysmimjbbunezgwcdpe
ohvflkatjsciqjxbunezgwcdpa
ohvxlkatysmitjxbunezswcdpr
ohvslkatfsmiqjxbunezgwudpr
ohvflkatysmiqexbugezgwcdnr
onvflkatysmiqjxkunezgtcdpr
fhsflkalysmiqjxbunezgwcdpr
oyvflkatysmiqjobxnezgwcdpr
ohvflkatysmiqjxbunezswgdvr
phvflkatyymiqjxvunezgwcdpr
oivflzutysmiqjxbunezgwcdpr
ohvflkftysmiqjxbunezkwcopr
ohvflkatysmwnjxbunezgwcdpp
ohvflkatysmiqkxcunezgwndpr
phvklkatysmiqjhbunezgwcdpr
ohvflrawysmiqjxbunhzgwcdpr
ohvflkatysmiqjxbunecgwcdig
ohvflpakysmiqjxbunezgwrdpr
odvflkatykmiqjxbunezglcdpr
ohtflkatysiiqjxblnezgwcdpr
lhvfpkatysmiqjxbupezgwcdpr
ohvflkatdsmiqjpbunezgwcdps
ohvflkztysmiqjxvunezgwjdpr
ohvflbatysmxqoxbunezgwcdpr
ohvklkaigsmiqjxbunezgwcdpr
ohvfgkawysmiqjxbunezgwcdur
ohvflkatyskpqjlbunezgwcdpr
ohvflkatyqmiqjhbupezgwcdpr
ohqflkatysmiqjxzonezgwcdpr
ohxfnkatyymiqjxbunezgwcdpr
ohmflkatpsmiqjxbunezgwcdpw
ohvflkatysmiqjibnnewgwcdpr
vevflkatysmiqjxbunezgwcypr
ohvflkatydmiqwxbungzgwcdpr
ohsrlkatysmiqjxbcnezgwcdpr
ohvflkptyvmiqexbunezgwcdpr
opzflkatysmiqjxrunezgwcdpr
ohvflkitysmiqjxcunezgwcmpr
ohvflkatysmhhjxblnezgwcdpr
ohvflkatysfiqjxbunrzgwmdpr
ohvflkatyamibjxbunezgwcdpf
ohvflkalysmigjxbunezggcdpr
ohvflkatwsmisjxbunezgdcdpr
dhvflkatysmlqjxbunszgwcdpr
ohvflkatysmiqjxbueeygwcbpr
ohvflkatgsmiqjnbunezhwcdpr
svvflkatysmiqjxbunezgwckpr
opvflkatysmiqpxbufezgwcdpr
ohnvlkatysmiqjxbunezglcdpr
phvflkutysjiqjxbunezgwcdpr
ohvflabtysmiqjjbunezgwcdpr
ouvflkatysmiqjsbunezgwcdpk
osvflkatysmijjxbunezgwcypr
owvflkatysmiqjxbukxzgwcdpr
ohvfliatvsmiljxbunezgwcdpr
ohvflkatysmiqjxbumezbwtdpr
ohvflkatyfcicjxbunezgwcdpr
ohvflkatysmiqldbunezgfcdpr
oqvflkatysmiqixkunezgwcdpr
ohvflkatysmiqjxbulezgicdpe
ohvflkatysmiqjxbuniegwcdpl
ohvflkatysmiqjwbunbzgwcdhr
ohvflkatysmiqjdbunezgwwdkr
ohqflkytysmiqjxbunezgwcdpc
ohvflkatysmigjxbunezqwwdpr
ohvfloatysmiqjpbumezgwcdpr
ohvklkathkmiqjxbunezgwcdpr
ohvflkstjsmiqjxbunezgwctpr
ohvvlkatysmiqjxbunewgwcdir
ohnflkatysmiqjxbunszgwcdlr
ohvflkatysmnqjxbunezgxcdlr
ohvfrkatysmiqjxbonezgwcdor
ihvflkatysmiqjxbuneogwcxpr
ohvflkatysmiqjxbunecgwcccr
owvflkatysmivjxbunezgwjdpr
ohvflkgtysmiqjxbunczhwcdpr
ohyqlkatysmiqjxbunezgwcypr
ohvflkatysmiqjvbunezuwcdpw
ohvflkathsmiqmxbuoezgwcdpr
ehvjlkajysmiqjxbunezgwcdpr
ohvflkltysmiqjxblnezgwjdpr
oovflkvtfsmiqjxbunezgwcdpr
olvfzkatysmiqjxyunezgwcdpr
ohvflkatysqitjxbunezgncdpr
yhvflkatysmkqjxbunazgwcdpr
zlvolkatysmiqjxbunezgwcdpr
ohvflpatysmiqjxbunezgwcapb
ohvflkatysmuqjxbunezgfcdur
";
