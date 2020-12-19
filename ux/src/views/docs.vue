<template>
  <div class="docs prose">
    <h1>Vault of Cardboard Query Language</h1>
    <p>This section describes how to write queries against the Vault.</p>

    <p>The simplest of Vault queries consist of single unquoted words
    (i.e.: <kbd>counterspell</kbd>), and quoted strings (i.e.: <kbd>"Blur Sliver"</kbd>).
    These queries match cards based on their names.  The unquoted
    variety is case-insensitive, but matches whole words &mdash; <kbd>ral</kbd>
    would match <em>Ral, Caller of Storms</em>, but not <em>Baral, Chief of
    Compliance</em>.</p>

    <p>More advanced expressions exist for matching against other parts
    of the card.  For example, <kbd>color:W</kbd> is an expression that matches
    cards whose color identity includes white.</p>

    <p>Here's a list of all the expression types currently supported by
    the Vault interpreter.  Feel free to skim this list upon first
    encountering it, and then return once you get to see how
    expressions can be combined!</p>

    <table class="keywords">
      <thead><tr>
        <th>Expression</th>
        <th>How it works</th>
        <th>Example</th>
      </tr></thead>
      <tbody>
        <tr>
          <td>activate</td>
          <td>Match cards based on the text of their activated abilities.</td>
          <td>activate:gains flying</td>
        </tr>
        <tr>
          <td>artist</td>
          <td>Match cards based on the name of the artist.  Art matters.</td>
          <td>artist:Poole</td>
        </tr>
        <tr>
          <td>border</td>
          <td>Match cards based on their border color / presence.</td>
          <td>border:black<br>border:none</td>
        </tr>
        <tr>
          <td>card</td>
          <td>Match cards based on print ID, oracle ID, or collector number.</td>
          <td>card:42</td>
        </tr>
        <tr>
          <td>cmc</td>
          <td>Match cards based on their converted mana cost.</td>
          <td>cmc:2+</td>
        </tr>
        <tr>
          <td>color</td>
          <td>Match cards based on their color identity.  Color names (<strong>blue</strong>, <strong>red</strong>, etc.) will match cards with only that color identity.  WUBRG combinations perform partial matches (<strong>UR</strong> matches a card in blue, red, <em>and</em> green, for example).  You can also use color pair / triple names, like <strong>Izzet</strong> or <strong>Bant</strong>, for exact matching.</td>
          <td>color:red<br>color:UR<br>color:izzet</td>
        </tr>
        <tr>
          <td>cpt</td>
          <td>Match (creature) cards based on their combined power + toughness values.</td>
          <td>cpt:&lt;4</td>
        </tr>
        <tr>
          <td>date</td>
          <td>Match cards based on when they were released.</td>
          <td>date:1996+<br>date:20200424</td>
        </tr>
        <tr>
          <td>discard</td>
          <td>Match cards based on their activated abilities that you pay for by discarding cards.</td>
          <td>discard:draw</td>
        </tr>
        <tr>
          <td>equip</td>
          <td>Match (equipment artifact) cards based on their cost to equip.</td>
          <td>equip:2+</td>
        </tr>
        <tr>
          <td>exile</td>
          <td>Match cards based on their activated abilities that you pay for by exiling cards or permanents.</td>
          <td>exile:life</td>
        </tr>
        <tr>
          <td>flavor</td>
          <td>Match cards based on the contents of their flavor text.</td>
          <td>flavor:Niv</td>
        </tr>
        <tr>
          <td>frame</td>
          <td>Match cards (as printed) based on their frame characteristics.  This is how you differentiate older cards
          from newer reprints, and also how you find or exclude alternate frames like showcase cards, extended art
          variants, etc.<br><br>
          Valid epoch-based frames are:
          <ul>
            <li><strong>1993</strong> The original layout, from Alpha through Alliances.</li>
            <li><strong>1997</strong> The revised frame, in use from Mirage (late '96) to Scourge.</li>
            <li><strong>2003</strong> (also called <strong>modern</strong>) The "modern" frame, introduced
            in 8th Edition and in use until Core 2015.</li>
            <li><strong>2015</strong> (also called <strong>current</strong>) The frame for new sets, printed today.</li>
            <li><strong>future</strong> The time-shifted frame from Future Sight (and not seen before or since).</li>
          </ul>
          <!--
          <br><br>
          Besides the base frames, there are several compositional frame <em>attributes</em> that can also be specified
          with the <tt>frame</tt> qualifier:
          <ul>
            <li><strong>color-shifted</strong> Planar Chaos had color-shifted cards.</li>
            <li><strong>companion</strong> Ikoria introduced the concept of companion, with a new frame treatment.</li>
            <li><strong>compass</strong> Ixalan gave as the <em>explore</em> mechanic, and with it, the transforming compass frame.</li>
            <li><strong>devoid</strong> Zendikar gave us the Eldrazi, the first non-artifact colorless spells.</li>
            <li><strong>draft</strong>Conspiracy (and Conspiracy 2) introduced draft-specicif cards like Schemes.  This matches those.</li>
            <li><strong>extended-art</strong> With so-called <em>extended art</em> cards, the art is stretched to beyond the normal borders of a Magic card.</li>
            <li><strong>legendary</strong> The legendary crown, that appears on more recent <em>legendary creature</em> permanent cards.
            <li><strong>miracle</strong> Innistrad gave us the <em>miracle</em> mechanic, with its own sunburst frame.</li>
            <li><strong>nyx</strong> <em>Nyx-touched</em> cards from the plane of Theros have a different background color texture.</li>
            <li><strong>showcase</strong> The showcase frame features new art, in a completely different card frame.</li>
            <li><strong>tombstone</strong> Introduced in Odyssey, the <em>flashback</em> mechanic saw cards gain a small tombstone in the top left corner.</li>
          </ul>
          -->
          </td>
          <td>frame:1997</td>
        </tr>
        <tr>
          <td>layout</td>
          <td>Match cards based on their form factor (i.e. <tt>normal</tt>, or <tt>transforms</tt>).</td>
          <td>layout:transform</td>
        </tr>
        <tr>
          <td>legal</td>
          <td>Match cards that are legal in the given format.<br><br>Valid values are:
          <ul>
            <li><strong>1v1</strong> (2-player EDH)</li>
            <li><strong>edh</strong> or <strong>commander</strong> (multiplayer EDH)</li>
            <li><strong>legacy</strong> (Legacy)</li>
            <li><strong>modern</strong> (8th ed + Mirrodin onward)</li>
            <li><strong>standard</strong> (formerly Type 2).</li>
            <li><strong>vintage</strong> (formerly Type 1).</li>
          </ul></td>
          <td>legal:standard</td>
        </tr>
        <tr>
          <td>name</td>
          <td>Match cards based on their name.</td>
          <td>name:Bolt</td>
        </tr>
        <tr>
          <td>oracle</td>
          <td>Match cards based on the contents of their oracle text.</td>
          <td>oracle:draw</td>
        </tr>
        <tr>
          <td>own</td>
          <td>Match cards based on how many you have in your collection.</td>
          <td>own:4+</td>
        </tr>
        <tr>
          <td>power</td>
          <td>Match (creature) cards based on their power / attack value.</td>
          <td>power:2+</td>
        </tr>
        <tr>
          <td>pt</td>
          <td>Match (creature) cards based on their literal power/toughness.</td>
          <td>pt:2/3</td>
        </tr>
        <tr>
          <td>ptr</td>
          <td>Match (creature) cards based on the ratio of power:toughness.</td>
          <td>ptr:1.5+</td>
        </tr>
        <tr>
          <td>rarity</td>
          <td>Match cards based on their commonality / rarity.</td>
          <td>rarity:mythic</td>
        </tr>
        <tr>
          <td>set</td>
          <td>Match cards based on the expansion set(s) they were printed in.</td>
          <td>set:XLN</td>
        </tr>
        <tr>
          <td>spotlight</td>
          <td>Matches cards that are <strong>Story Spotlight</strong> cards.</td>
          <td>spotlight:y</td>
        </tr>
        <tr>
          <td>tap</td>
          <td>Match cards based on their activated abilities that require a tap.</td>
          <td>tap:destroy</td>
        </tr>
        <tr>
          <td>toughness</td>
          <td>Match (creature) cards based on their toughness / defense value.</td>
          <td>toughness:&lt;5</td>
        </tr>
        <tr>
          <td>type</td>
          <td>Match cards based on their card type.</td>
          <td>type:land</td>
        </tr>
        <tr>
          <td>unique</td>
          <td>Filter results based on card text or art uniqueness.
              See below for more information and some caveats.</td>
          <td>unique:card<br>unique:art</td>
        </tr>
        <tr>
          <td>untap</td>
          <td>Match cards based on their activated abilities that require an untap.</td>
          <td>untap:add</td>
        </tr>
        <tr>
          <td>usd</td>
          <td>Match cards based on their current value/price (per Scryfall)</td>
          <td>usd:&lt;5.25</td>
        </tr>
      </tbody>
    </table>

    <p>Several of these typed expressions are so common that VCB has
    either a shorthand or keyword notation for them.</p>

    <table class="shorthand">
      <thead><tr>
        <th>Shorthand</th>
        <th>Equivalent to</th>
        <th>Example</th>
      </tr></thead>

      <tbody>
        <tr>
          <td>=X</td>
          <td>rarity:X</td>
          <td><kbd>=mythic</kbd> will find all Mythic Rares.</td>
        </tr>
        <tr>
          <td>@X</td>
          <td>color:X</td>
          <td><kbd>@WB</kbd> will find dual white/black cards.</td>
        </tr>
        <tr>
          <td>+X</td>
          <td>oracle:X</td>
          <td><kbd>+draw</kbd> finds cards that have the word "draw" in their rules text.</td>
        </tr>
        <tr>
          <td>owned</td>
          <td>own:1+</td>
          <td><kbd>owned</kbd> finds all of your cards.</td>
        </tr>
        <tr>
          <td>have</td>
          <td>own:1+</td>
          <td><kbd>have</kbd> is an alias for <kbd>owned</kbd>.</td>
        </tr>
        <tr>
          <td>need</td>
          <td>own:0</td>
          <td><kbd>need</kbd> finds all of the cards you don't own (which you clearly <strong>need</strong>).</td>
        </tr>
      </tbody>
    </table>

    <p class="note"><span>Note:</span>
    if you want to match keywords as a name, for instance if you were
    looking for <em>Hour of Need</em>, you can use the <kbd>name:</kbd> expression, as in
    <kbd>name:"Need"</kbd>.  (Due to a technical limitation that should be fixed soon,
    you do have to quote the value, which means capitalization matters!)</p>

    <p>For anything but the simplest queries, you're going to want to
    combine these expressions into larger aggregate expressions.  For these, you
    need the <em>logical operators</em>: <kbd>and</kbd>, <kbd>or</kbd>, and <kbd>not</kbd>.</p>

    <p><kbd>and</kbd> works by checking both sub-expressions; if those both match, the
    expression as a whole is a match.  <kbd>or</kbd> considers the whole expression a
    match if <em>either</em> sub-expression is a match.  This mirrors how we use the
    words "and" / "or" in English.</p>

    <p>For example, there are a lot of legendary creatures.  To build a blue / black
    commander (EDH) deck,  I need to find an interesting legendary creature who
    is both blue and black.  That's two expressions, stitched together with an
    <kbd>and</kbd>:</p>

    <pre><code class="query">type:legendary and color:UB</code></pre>

    <p>The <kbd>not</kbd> operator lets you negate an assertion.  Commander doesn't
    generally allow Planeswalkers to serve as Commanders, and since most walkers
    are listed as "Legendary Planeswalkers", I need to amend my query to
    exclude all mention of the sub-type:</p>

    <pre><code class="query">type:legendary and color:UB and not type:planeswalker</code></pre>

    <p>Much better.</p>

    <p>In the interest of reducing keystrokes, <kbd>!</kbd> is an alias of <kbd>not</kbd>; forgive
    me, this is my programmer roots showing.  A nice compact query that takes
    advantage of all this terseness is:</p>

    <pre><code class="query">owned and !=common</code></pre>

    <p>... which finds all of the uncommons, rares, and mythic rares in your
    collection.</p>

    <h2>Uniqueness</h2>
    <p>By default, Vault of Cardboard will return all prints and editions of a <em>card</em>.
    For example, if you search for <kbd>counterspell</kbd>, you'll get a bunch of them, mostly
    reprints.  If you don't care about specific editions, and are just looking for the general
    counterspell card, you can add <kbd>and unique</kbd> to the end of your search query.</p>

    <p>There are two modes for the <kbd>unique</kbd> predicate: oracle text and art.</p>

    <p>For <strong>oracle text</strong>, use <kbd>unique:card</kbd>, or the briefer shortcut
    <kbd>unique</kbd>.  As it compiles the search results, Vault of Cardboard will omit duplicates
    of reprinted cards.  Note that this may not always return the <em>original</em> printing.</p>

    <p>Sometimes cards get a reprint with new art.  To find all unique illustrations, you can use
    the <strong>card art</strong> mode: <kbd>unique:art</kbd>.</p>

    <p>This type of filtering is different from all of the other query predicates, since it
    has to take into account all of the cards seen so far.  As such, you will probably see some
    strange behavior if you try to nest the uniqueness predicate inside of a subquery.</p>

    <p>For 99% of use cases, you'll want to just tack on a <kbd>and unique</kbd> to the end of
    your query.</p>

    <h2>A Note About Precedence</h2>
    <p>Finally, we need to talk about precedence.  Whenever you chain multiple
    <kbd>and</kbd>'s and <kbd>or</kbd>'s together, you need to consider what you actually mean.
    For example:</p>

    <pre><code class="query">owned and =rare or =mythic</code></pre>

    <p>The <em>intent</em> behind this query was to find all my rares.  Unfortunately, the
    precedence rules of the query language <em>bind</em> the <kbd>owned</kbd> and <kbd>=rare</kbd>
    sub-expressions together, and then checks the <kbd>=mythic</kbd> status.  The upshot
    is that what I actually get is all of my owned rares, and every mythic rare
    ever printed.</p>

    <p>I could reword the query to be:</p>

    <pre><code class="query">=rare or =mythic and owned</code></pre>

    <p>(in general, <kbd>and owned</kbd> on the end will almost always do what you want)</p>

    <p>Luckily, you can use parentheses to enforce the precedence you want, without
    having to reword:</p>

    <pre><code class="query">owned and (=mythic or =rare)</code></pre>

    <p>This causes the rarity expressions to be evaluated, and then checked against
    ownership status.  Just like math class!</p>

    <h2>A Query Cookbook</h2>

    <p>If you prefer to learn by doing, or just want to try some things
    out, this is the section for you.</p>

    <p><strong>What do I own from <em>Dominaria</em>?</strong></p>

    <pre><code class="query">set:DOM and owned</code></pre>

    <p><strong>What red playsets do I own?</strong> (i.e. for standard deck
    construction purposes)</p>

    <pre><code class="query">@R and own:4+</code></pre>

    <p><strong>What taplands exist?</strong></p>

    <pre><code class="query">type:Land and +tapped</code></pre>

    <p><strong>I would like to drool over Zendikar lands please</strong></p>

    <pre><code class="query">set:ZEN or set:BFZ and type:land and type:basic</code></pre>

    <p>or (if you like parentheticals (as I do)):</p>

    <pre><code class="query">(set:ZEN or set:BFZ) and (type:land and type:basic)</code></pre>

    <p><strong>Oh man do I love a good life transfer card!</strong></p>

    <pre><code class="query">+gains? and +loses?</code></pre>

    <p><strong>What white weenies have lifelink?</strong></p>

    <pre><code class="query">@W and +lifelink and pt:1/1</code></pre>

    <p><strong>What GRN creatures had more attack than defense?</strong></p>

    <pre><code class="query">set:GRN and ptr:&gt;1</code></pre>
  </div>
</template>
