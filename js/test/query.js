const expect = require('chai').expect;

const cardboard = require('../index.js');

describe('Cardboard Query Parser', () => {
  let test = (q, want) => { return () => expect(cardboard.Query.parse(q).toString()).to.equal(want) };

  it('should infer case-insensitive name pattern-matching by default',
    test('foo', '(NAME /\\bfoo\\b/i)'));

  it('should infer case-sensitive name pattern-matching when quoted',
    test('"foo"', '(NAME /\\bfoo\\b/)'));

  it('should treat multiple whitespace-separated keywords as a single name match',
    test('a b c', '(NAME /\\ba b c\\b/i)'));

  "WUBRG".split('').forEach(c =>
    it('should parse references to single-color {'+c+'} identity',
      test('@'+c, '(COLOR '+c+')'))
  );

  it('should handle rarity-based queries',
    test('=uncommon', '(RARITY 2)'));

  it('should handle set membership queries',
    test('set:MIR', '(SET MIR)'));

  it('should handle uniqueness constraints',
    test('unique:art', '(UNIQUE art)'));

  [
    'type',
    'oracle',
    'flavor',
    'oracle',
    'activate',
    'discard',
    'tap',
    'untap'
  ].forEach(x =>
    it('should handle '+x+' queries (as partial-match)',
      test(x+':foo', '('+x.toUpperCase()+' /\\bfoo\\b/i)')));
  [
    'layout',
  ].forEach(x =>
    it('should handle '+x+' queries (as keyword-match)',
      test(x+':foo', '('+x.toUpperCase()+' foo)')));

  [
    'equip',
    'own',
    'usd',
    'cmc',
    'p',
    't',
    'cpt',
    'ptr'
  ].forEach(x => {
    it('should handle '+x+':2 queries as numeric (equality)',
      test(x+':2', '('+x.toUpperCase()+' 2)'));
    it('should handle '+x+':2+ queries as numeric (greater than or equal to)',
      test(x+':2+', '('+x.toUpperCase()+' 2+)'));
    it('should handle '+x+':>2 queries as numeric (greater than)',
      test(x+':>2', '('+x.toUpperCase()+' >2)'));
    it('should handle '+x+':>=2 queries as numeric (greater than or equal to)',
      test(x+':>=2', '('+x.toUpperCase()+' >=2)'));
    it('should handle '+x+':<2 queries as numeric (less than)',
      test(x+':<2', '('+x.toUpperCase()+' <2)'));
    it('should handle '+x+':<=2 queries as numeric (less than or equal to)',
      test(x+':<=2', '('+x.toUpperCase()+' <=2)'));
  });

  [
    'fullart',
    'oversized',
    'variant',
    'spotlight',
    'reserved',
    'reprint'
  ].forEach(x => {
    it('should handle '+x+':yes queries',
      test(x+':yes', '('+x.toUpperCase()+' yes)'));
    it('should handle '+x+':y queries',
      test(x+':y', '('+x.toUpperCase()+' yes)'));
    it('should handle '+x+':no queries',
      test(x+':no', '('+x.toUpperCase()+' no)'));
    it('should handle '+x+':n queries',
      test(x+':n', '('+x.toUpperCase()+' no)'));
    it('should handle '+x+':1 queries',
      test(x+':1', '('+x.toUpperCase()+' yes)'));
  });

  it('should handle negation of queries',
    test('!cmc:2', '(NOT (CMC 2))'));

  it('should handle logical AND composition',
    test('own:2 and cmc:2', '(AND (OWN 2) (CMC 2))'));

  it('should handle logical OR composition',
    test('own:2 or cmc:2', '(OR (OWN 2) (CMC 2))'));

  it('should handle un-grouped AND + OR branching',
    test('@R or @B and @U', '(AND (OR (COLOR R) (COLOR B)) (COLOR U))'));

  it('should handle explicitly-grouped AND + OR branching',
    test('@R or (@B and @U)', '(OR (COLOR R) (AND (COLOR B) (COLOR U)))'));

  it('should handle grouped and negated sub-queries',
    test('@R or (!@U and !@B)', '(OR (COLOR R) (AND (NOT (COLOR U)) (NOT (COLOR B))))'));

  it('should handle brawl legality queries',      test('legal:brawl',      '(LEGAL B)'));
  it('should handle edh legality queries',        test('legal:edh',        '(LEGAL E)'));
  it('should handle commander legality queries',  test('legal:commander',  '(LEGAL E)'));
  it('should handle duel legality queries',       test('legal:duel',       '(LEGAL d)'));
  it('should handle frontier legality queries',   test('legal:frontier',   '(LEGAL j)'));
  it('should handle future legality queries',     test('legal:future',     '(LEGAL f)'));
  it('should handle historic legality queries',   test('legal:historic',   '(LEGAL h)'));
  it('should handle legacy legality queries',     test('legal:legacy',     '(LEGAL l)'));
  it('should handle modern legality queries',     test('legal:modern',     '(LEGAL m)'));
  it('should handle old-school legality queries', test('legal:old-school', '(LEGAL o)'));
  it('should handle pauper legality queries',     test('legal:pauper',     '(LEGAL P)'));
  it('should handle penny legality queries',      test('legal:penny',      '(LEGAL $)'));
  it('should handle pioneer legality queries',    test('legal:pioneer',    '(LEGAL p)'));
  it('should handle standard legality queries',   test('legal:standard',   '(LEGAL s)'));
  it('should handle vintage legality queries',    test('legal:vintage',    '(LEGAL v)'));

  it('should handle 1993 frame queries', test('frame:1993', '(FRAME 3)'));
  it('should handle "93" frame alias',   test('frame:93',   '(FRAME 3)'));

  it('should handle 1997 frame queries', test('frame:1997', '(FRAME 7)'));
  it('should handle "97" frame alias',   test('frame:97',   '(FRAME 7)'));

  it('should handle 2003 frame queries',   test('frame:2003',   '(FRAME M)'));
  it('should handle "03" frame alias',     test('frame:03',     '(FRAME M)'));
  it('should handle "modern" frame alias', test('frame:modern', '(FRAME M)'));

  it('should handle 2015 frame queries',    test('frame:2015',    '(FRAME N)'));
  it('should handle "15" frame alias',      test('frame:15',      '(FRAME N)'));
  it('should handle "current" frame alias', test('frame:current', '(FRAME N)'));

  it('should handle future frame queries',       test('frame:future',       '(FRAME F)'));
  it('should handle "timeshifted" frame alias',  test('frame:timeshifted',  '(FRAME F)'));
  it('should handle "time-shifted" frame alias', test('frame:time-shifted', '(FRAME F)'));

  it('should handle "93+" frame range',    test('frame:93+',    '(FRAME 37MNF)'));
  it('should handle "1993+" frame range',  test('frame:1993+',  '(FRAME 37MNF)'));
  it('should handle ">93" frame range',    test('frame:>93',    '(FRAME 7MNF)'));
  it('should handle ">1993" frame range',  test('frame:>1993',  '(FRAME 7MNF)'));
  it('should handle ">=93" frame range',   test('frame:>=93',   '(FRAME 37MNF)'));
  it('should handle ">=1993" frame range', test('frame:>=1993', '(FRAME 37MNF)'));
  it('should handle "<93" frame range',    test('frame:<93',    '(FRAME )'));
  it('should handle "<1993" frame range',  test('frame:<1993',  '(FRAME )'));
  it('should handle "<=93" frame range',   test('frame:<=93',   '(FRAME 3)'));
  it('should handle "<=1993" frame range', test('frame:<=1993', '(FRAME 3)'));

  it('should handle "97+" frame range',    test('frame:97+',    '(FRAME 7MNF)'));
  it('should handle "1997+" frame range',  test('frame:1997+',  '(FRAME 7MNF)'));
  it('should handle ">97" frame range',    test('frame:>97',    '(FRAME MNF)'));
  it('should handle ">1997" frame range',  test('frame:>1997',  '(FRAME MNF)'));
  it('should handle ">=97" frame range',   test('frame:>=97',   '(FRAME 7MNF)'));
  it('should handle ">=1997" frame range', test('frame:>=1997', '(FRAME 7MNF)'));
  it('should handle "<97" frame range',    test('frame:<97',    '(FRAME 3)'));
  it('should handle "<1997" frame range',  test('frame:<1997',  '(FRAME 3)'));
  it('should handle "<=97" frame range',   test('frame:<=97',   '(FRAME 37)'));
  it('should handle "<=1997" frame range', test('frame:<=1997', '(FRAME 37)'));

  it('should handle "03+" frame range',      test('frame:03+',      '(FRAME MNF)'));
  it('should handle "2003+" frame range',    test('frame:2003+',    '(FRAME MNF)'));
  it('should handle "modern+" frame range',  test('frame:modern+',  '(FRAME MNF)'));
  it('should handle ">03" frame range',      test('frame:>03',      '(FRAME NF)'));
  it('should handle ">2003" frame range',    test('frame:>2003',    '(FRAME NF)'));
  it('should handle ">modern" frame range',  test('frame:>modern',  '(FRAME NF)'));
  it('should handle ">=03" frame range',     test('frame:>=03',     '(FRAME MNF)'));
  it('should handle ">=2003" frame range',   test('frame:>=2003',   '(FRAME MNF)'));
  it('should handle ">=modern" frame range', test('frame:>=modern', '(FRAME MNF)'));
  it('should handle "<03" frame range',      test('frame:<03',      '(FRAME 37)'));
  it('should handle "<2003" frame range',    test('frame:<2003',    '(FRAME 37)'));
  it('should handle "<modern" frame range',  test('frame:<modern',  '(FRAME 37)'));
  it('should handle "<=03" frame range',     test('frame:<=03',     '(FRAME 37M)'));
  it('should handle "<=2003" frame range',   test('frame:<=2003',   '(FRAME 37M)'));
  it('should handle "<=modern" frame range', test('frame:<=modern', '(FRAME 37M)'));

  it('should handle "15+" frame range',       test('frame:15+',       '(FRAME NF)'));
  it('should handle "2015+" frame range',     test('frame:2015+',     '(FRAME NF)'));
  it('should handle "current+" frame range',  test('frame:current+',  '(FRAME NF)'));
  it('should handle ">15" frame range',       test('frame:>15',       '(FRAME F)'));
  it('should handle ">2015" frame range',     test('frame:>2015',     '(FRAME F)'));
  it('should handle ">current" frame range',  test('frame:>current',  '(FRAME F)'));
  it('should handle ">=15" frame range',      test('frame:>=15',      '(FRAME NF)'));
  it('should handle ">=2015" frame range',    test('frame:>=2015',    '(FRAME NF)'));
  it('should handle ">=current" frame range', test('frame:>=current', '(FRAME NF)'));
  it('should handle "<15" frame range',       test('frame:<15',       '(FRAME 37M)'));
  it('should handle "<2015" frame range',     test('frame:<2015',     '(FRAME 37M)'));
  it('should handle "<current" frame range',  test('frame:<current',  '(FRAME 37M)'));
  it('should handle "<=15" frame range',      test('frame:<=15',      '(FRAME 37MN)'));
  it('should handle "<=2015" frame range',    test('frame:<=2015',    '(FRAME 37MN)'));
  it('should handle "<=current" frame range', test('frame:<=current', '(FRAME 37MN)'));

  it('should handle "future+" frame range',  test('frame:future+',  '(FRAME F)'));
  it('should handle ">future" frame range',  test('frame:>future',  '(FRAME )'));
  it('should handle ">=future" frame range', test('frame:>=future', '(FRAME F)'));
  it('should handle "<future" frame range',  test('frame:<future',  '(FRAME 37MN)'));
  it('should handle "<=future" frame range', test('frame:<=future', '(FRAME 37MNF)'));

  it('should handle "old" frame range alias',  test('frame:old',  '(FRAME 37)'));
  it('should handle "new" frame range alias',  test('frame:new',  '(FRAME MNF)'));

  it('should handle legendary frame queries',    test('frame:legendary',     '(FRAME L)'));
  it('should handle miracle frame queries',      test('frame:miracle',       '(FRAME m)'));
  it('should handle nyx frame queries',          test('frame:nyx',           '(FRAME n)'));
  it('should handle nyx-touched frame alias',    test('frame:nyx-touched',   '(FRAME n)'));
  it('should handle nyxtouched frame alias',     test('frame:nyxtouched',    '(FRAME n)'));
  it('should handle draft frame queries',        test('frame:draft',         '(FRAME D)'));
  it('should handle devoid frame queries',       test('frame:devoid',        '(FRAME d)'));
  it('should handle tombstone frame queries',    test('frame:tombstone',     '(FRAME t)'));
  it('should handle flashback frame alias',      test('frame:flashback',     '(FRAME t)'));
  it('should handle colorshifted frame queries', test('frame:colorshifted',  '(FRAME $)'));
  it('should handle color-shifted frame alias',  test('frame:color-shifted', '(FRAME $)'));
  it('should handle showcase frame queries',     test('frame:showcase',      '(FRAME s)'));
  it('should handle compass frame queries',      test('frame:compass',       '(FRAME c)'));
  it('should handle extended frame queries',     test('frame:extended',      '(FRAME +)'));
  it('should handle extended-art frame alias',   test('frame:extended-art',  '(FRAME +)'));
  it('should handle extendedart frame alias',    test('frame:extendedart',   '(FRAME +)'));
  it('should handle stretched frame alias',      test('frame:stretched',     '(FRAME +)'));
  it('should handle companion frame queries',    test('frame:companion',     '(FRAME C)'));

  it('should handle border queries', test('border:black', '(BORDER black)'));

  it('should handle card id/number queries', test('card:3b', '(CARD 3b)'));
});

describe('Individual Card Querying', () => {
  let $c = {
    id        : '51657034-2c30-40a2-a215-a00277f01642',
    oid       : 'c5661500-c48f-4f6d-bbe7-c8bd7118d862',
    flags     : '1EdlB$v',
    artist    : 'Pete Venters',
    layout    : 'normal',
    number    : '196',
    rarity    : 'common', // from flags
    image     : 'MIR/MIR-51657034-2c30-40a2-a215-a00277f01642.jpg',
    name      : 'Talruum Minotaur',
    type      : 'Creature — Minotaur Berserker',
    oracle    : 'Haste',
    cmc       : 4,
    cost      : '{2}{R}{R}',
    color     : 'R',
    price     : 1.15, // price-fixing
    owned     : 4,
    flavor    : "Don't insult a Talruum unless your mount is swift. —Suq'Ata saying",
    power     : '3',
    toughness : '3',
    pt        : '3/3',
    art       : 'decd9cb4-ef84-4a79-b8bf-e3151535ccda',
    back      : '',
    set: {
      name: 'Mirage',
      code: 'MIR',
      total: 350
    }
  };
  let should_match = (m,q) => it('should match '+m+' ('+q+')',
    () => expect(cardboard.Query.parse(q).match($c)).to.be.true);

  should_match('based on name predicate',      'talruum');
  should_match('based on oracle predicate',    '+haste');
  should_match('based on flavor predicate',    'flavor:insult');
  should_match('based on set predicate',       'set:MIR');
  should_match('based on type predicate',      'type:creature');
  should_match('based on artist predicate',    'artist:Pete');
  should_match('based on color predicate',     '@R');
  should_match('based on cmc predicate',       'cmc:4');
  should_match('based on layout predicate',    'layout:normal');
  should_match('based on rarity predicate',    '=common');
  should_match('based on power predicate',     'p:3');
  should_match('based on toughness predicate', 't:3');
  should_match('based on total p/t',           'pt:3/3');
  should_match('based on combined p+t',        'cpt:6');
  should_match('based on ratio of p:t',        'ptr:1');
  should_match('based on legality predicate',  'legal:vintage');

  should_match('based on oversized predicate', 'oversized:no');
  should_match('based on fullart predicate',   'fullart:no');
  should_match('based on spotlight predicate', 'spotlight:no');
  should_match('based on variant predicate',   'variant:no');
  should_match('based on reserved predicate',  'reserved:no');
  should_match('based on reprint predicate',   'reprint:no');

  should_match('based on (negated) reprint predicate', '!reprint');

  should_match('based on usd predicate',       'usd:1.15');
  should_match('based on usd predicate',       'usd:1+');
  should_match('based on usd predicate',       'usd:>1');
  should_match('based on usd predicate',       'usd:<2');
  should_match('based on usd predicate',       'usd:<=2');

  should_match('based on owned predicate',     'own:4');

  should_match('based on negated equip predicate', '!equip:2');

  should_match('based on a complicated query',
    'Talruum and Minotaur and (@U or @R) and !=mythic');

  it('should allowi matching cards based on equip costs', () => {
    let c = { oracle: 'Equip {2}.' };
    expect(cardboard.Query.parse('equip:2').match(c)).to.be.true;
    expect(cardboard.Query.parse('equip:1+').match(c)).to.be.true;
    expect(cardboard.Query.parse('equip:<=3').match(c)).to.be.true;
  });

  it('should allowi matching cards based on {T} activation', () => {
    let c = { oracle: '{T}: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('tap:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('tap:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('tap:').match(c)).to.be.true;
    expect(cardboard.Query.parse('tap: and +do the thing').match(c)).to.be.true;
  });

  it('should allowi matching cards based on {Q} activation', () => {
    let c = { oracle: '{Q}: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('untap:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('untap:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('untap:').match(c)).to.be.true;
    expect(cardboard.Query.parse('untap: and +do the thing').match(c)).to.be.true;
  });

  it('should allowi matching cards based on "sacrifice" activation', () => {
    let c = { oracle: 'Sacrifice a creature: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('sacrifice:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('sacrifice:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('sacrifice:').match(c)).to.be.true;
    expect(cardboard.Query.parse('sacrifice: and +do the thing').match(c)).to.be.true;
  });

  it('should allowi matching cards based on "discard" activation', () => {
    let c = { oracle: 'Discard a card: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('discard:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('discard:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('discard:').match(c)).to.be.true;
    expect(cardboard.Query.parse('discard: and +do the thing').match(c)).to.be.true;
  });

  it('should allowi matching cards based on "exile" activation', () => {
    let c = { oracle: 'Exile a token: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

    expect(cardboard.Query.parse('exile:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('exile:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('exile:').match(c)).to.be.true;
    expect(cardboard.Query.parse('exile: and +do the thing').match(c)).to.be.true;
  });

  it('should allowi matching cards based on "pay" activation', () => {
    let c = { oracle: 'Exile a token: Do the thing' };
    expect(cardboard.Query.parse('activate:do the thing').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:"Do the thing"').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate:').match(c)).to.be.true;
    expect(cardboard.Query.parse('activate: and +do the thing').match(c)).to.be.true;

//    expect(cardboard.Query.parse('pay:do the thing').match(c)).to.be.true;
//    expect(cardboard.Query.parse('pay:"Do the thing"').match(c)).to.be.true;
//    expect(cardboard.Query.parse('pay:').match(c)).to.be.true;
//    expect(cardboard.Query.parse('pay: and +do the thing').match(c)).to.be.true;
  });

  it('should allow matching cards based on card frame', () => {
    let c = { frame: '3' };
    expect(cardboard.Query.parse('frame:93').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:97').match(c)).to.be.false;

    c.frame = 'LCM';
    expect(cardboard.Query.parse('frame:companion').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:compass').match(c)).to.be.false;
  });

  it('should allow matching cards based on time-sensitive card frames', () => {
    let c = { frame: 'M' };
    expect(cardboard.Query.parse('frame:modern').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:modern+').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:97+').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:93+').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:<current').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:<1997').match(c)).to.be.false;
    expect(cardboard.Query.parse('frame:old').match(c)).to.be.false;
    expect(cardboard.Query.parse('frame:new').match(c)).to.be.true;
  });

  it('should allow matching cards based on "specialness" of frame', () => {
    let c = { frame: 'sN' };
    expect(cardboard.Query.parse('frame:special').match(c)).to.be.true;
    expect(cardboard.Query.parse('frame:normal').match(c)).to.be.false;
  });

  it('should allow matching of all *-shifted card frames', () => {
    let c = { frame: '$M' };
    expect(cardboard.Query.parse('frame:shifted').match(c)).to.be.true;

    c.frame = 'F';
    expect(cardboard.Query.parse('frame:shifted').match(c)).to.be.true;

    c.frame = '7';
    expect(cardboard.Query.parse('frame:shifted').match(c)).to.be.false;
  });

  it('should allow matching based on border color / presence', () => {
    let c = { border: 'black' };
    expect(cardboard.Query.parse('border:black').match(c)).to.be.true;
    expect(cardboard.Query.parse('border:BLACK').match(c)).to.be.true;
    expect(cardboard.Query.parse('!border:white').match(c)).to.be.true;
  });

  it('should allow matching based on card ID, oracle ID, and collector number', () => {
    let c = {
      id:     '8e2bce5d-283f-45aa-9201-a1bfc65add25',
      oid:    'a9a55a50-731c-4cdc-a09f-23aac0c5c0f5',
      number: '42'
    };
    expect(cardboard.Query.parse('card:8e2bce5d-283f-45aa-9201-a1bfc65add25').match(c)).to.be.true;
    expect(cardboard.Query.parse('card:a9a55a50-731c-4cdc-a09f-23aac0c5c0f5').match(c)).to.be.true;
    expect(cardboard.Query.parse('card:42').match(c)).to.be.true;

    expect(cardboard.Query.parse('card:1').match(c)).to.be.false;
    expect(cardboard.Query.parse('card:1b538f5b-aacf-40c9-aede-1d5cdbee55ce').match(c)).to.be.false;
  });
});
