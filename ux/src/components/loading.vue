<template>
  <div id="loading">
    <h1>Loading...</h1>
    <div class="marquee">
      <div v-for="(quote,i) in quotes" :key="i">
        <blockquote>
          <span v-html="quote.q"></span>
          <cite>&mdash; <span v-html="quote.c"></span></cite>
        </blockquote>
      </div>
    </div>
  </div>
</template>

<script>
let quotes = [
  /* flavor:canto or flavor:"Song" */
  /* Arbor Armament (DOM) */
  { q: "&ldquo;Llanowar's boughs are ever ready<br>To unleash an autumn of steel leaves.&rdquo;",
    c: "&ldquo;Song of Freyalise&rdquo;" },

  /* Blinding Light (MIR) */
  { q: "&ldquo;My shield will bear a shining sun so you will always be with me. / Inlaid with gold, it will shine like glowing embers.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Chariot of the Sun (MIR) */
  { q: "&ldquo;Sun follows Moon until she tires, then carries her until she's strong / and runs ahead of him again.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Congregate (A25) */
  { q: "&ldquo;In the gathering there is strength for all who founder, renewal for all who languish, love for all who sing.&rdquo;",
    c: "Song of All, canto 642" },

  /* Defensive Formation (USG) */
  { q: "&ldquo;Your enemies will pound upon the door of your defenses, but only you shall have the key, and it is the key of life.&rdquo;",
    c: "Song of All, canto 873" },

  /* Disenchant (A25) */
  { q: "&ldquo;The tools of evil are mere things. And like all things, they cannot last forever.&rdquo;",
    c: "Song of All, canto 881" },

  /* Early Harvest (MIR) */
  { q: "&ldquo;Tonight we'll eat a farewell feast. Cold corn porridge is not enough. / Let's peel papayas, pineapples, and mangoes, drink coconut milk, / and bake bananas.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Femeref Knight (MIR) */
  { q: "&ldquo;I will return / with lizard skins for your sandals. Paint your eyes black and wait for me.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Flare (MIR) */
  { q: "&ldquo;In the forest, fires light the sky as black clouds unfold their weight.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Invoke the Divine (DOM) */
  { q: "&ldquo;Let go of all that harms you. Cast your burdens into the darkness, and build for the faithful a house of light.&rdquo;",
    c: "Song of All, canto 1008" },

  /* Kukemssa Pirates (MIR) */
  { q: "&ldquo;. . . pirates gambled with a djinn and lost the thing more dear than gold.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* On Serra's Wings (DOM) */
  { q: "&ldquo;The spirit of Serra raised Brindri high and commanded her to keep the balance.&rdquo;",
    c: "Song of All, canto 524" },

  /* Pegasus Charger (USG) */
  { q: "&ldquo;The clouds came alive and dove to the earth! Hooves flashed among the dark army, who fled before the spectacle of fury.&rdquo;",
    c: "Song of All, canto 211" },

  /* Planar Birth (USG) */
  { q: "&ldquo;From womb of nothingness sprang this place of beauty, purity, and hope realized.&rdquo;",
    c: "Song of All, canto 3" },

  /* Sacred Mesa (MIR) */
  { q: "&ldquo;Do not go there, do not go / unless you rise on wings, unless you walk on hooves.&rdquo;",
    c: "&ldquo;Song to the Sun,&rdquo; Femeref song" },

  /* Serra Avenger (TSP) */
  { q: "&ldquo;Those who endure in the face of suffering, those whose faith shines long in evil days, they shall see salvation.&rdquo;",
    c: "Song of All, canto 904" },

  /* Serra's Embrace (USG) */
  { q: "&ldquo;Lifted beyond herself, for that battle Brindri was an angel of light and fury.&rdquo;",
    c: "Song of All, canto 524" },

  /* Unfulfilled Desires (MIR) */
  { q: "&ldquo;Like Day from Night, / I'll live my life apart from you, just glimpsing you across the sky, / because you cannot change, my dear, and nor can I.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Village Elder (MIR) */
  { q: "&ldquo;Enchant me with your tale-telling. Tell about Tree, Grass, River, and Wind. / Tell why Truth must fight with Falsehood, and why Truth will always win.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Voice of Grace (USG) */
  { q: "&ldquo;Opposite Law is Grace, and Grace must be preserved. If the strands of Grace are unraveled, its design will be lost, and the people with it.&rdquo;",
    c: "Song of All, canto 167" },

  /* Voice of Law (USG) */
  { q: "&ldquo;Life's balance is as a star: on one point is Law, and Law must be upheld. If the knots of order are loosened, chaos will spill through.&rdquo;",
    c: "Song of All, canto 167" },

  /* Wild Elephant (MIR) */
  { q: "&ldquo;I will tell my father's stories . . . . How the elephants trampled the leopard cub, and its father, though he knew, killed nine goats instead.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Wildgrowth Walker (XLN) */
  { q: "&ldquo;Hear me, stone, root, branch: be the fist of this land. Turn back those who trample upon your domain.&rdquo;",
    c: "Song of the Shaper" },

  /* Zebra Unicorn (MIR) */
  { q: "&ldquo;I'll capture gentle zebras / for your steeds and fill the stable with every kind of unicorn.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" },

  /* Zhalfirin Knight (MIR) */
  { q: "&ldquo;You returned a warrior. . . . Your hair was cut, your eye tattooed with the red triangle of war.&rdquo;",
    c: "&ldquo;Love Song of Night and Day&rdquo;" }
]

import _ from "@/lib/helpers"

export default {
  name: 'vcb-loading',
  props: {
    quotes: {
      type: Array,
      default: () => _.shuffle(quotes)
    }
  }
}
</script>

<style lang="scss" scoped>
@keyframes loading {
    0%    {opacity: 0.0}
    0.5%  {opacity: 1.0}
    3%    {opacity: 1.0}
    3.5%  {opacity: 0.0}
  100%    {opacity: 0.0}
}

h1 {
  font-size: 24pt;
  font-style: italic;
  text-align: left;
  width: 50vw;
  margin: 30vh auto 0em;
}

.marquee {
  position: relative;

  width: 50vw;
  margin: 10vh auto;
  height: 40vh;

  > div {
    position: absolute;
    top: 0; left: 0; right: 0;

    line-height: 1.25em;
    text-indent: -1ex;
    font-size: 18pt;
    font-style: italic;
    color: #777;

    blockquote {
      display: block;
      width: auto;
      cite {
        display: block;
        text-align: right;
      }
    }

    /* animation! */
    animation: 200s loading infinite linear;
    opacity: 0.0;

    &:nth-child(1)  { animation-delay:   0s }
    &:nth-child(2)  { animation-delay:   8s }
    &:nth-child(3)  { animation-delay:  16s }
    &:nth-child(4)  { animation-delay:  24s }
    &:nth-child(5)  { animation-delay:  32s }
    &:nth-child(6)  { animation-delay:  40s }
    &:nth-child(7)  { animation-delay:  48s }
    &:nth-child(8)  { animation-delay:  56s }
    &:nth-child(9)  { animation-delay:  64s }
    &:nth-child(10) { animation-delay:  72s }
    &:nth-child(11) { animation-delay:  80s }
    &:nth-child(12) { animation-delay:  88s }
    &:nth-child(13) { animation-delay:  96s }
    &:nth-child(14) { animation-delay: 104s }
    &:nth-child(15) { animation-delay: 112s }
    &:nth-child(16) { animation-delay: 120s }
    &:nth-child(17) { animation-delay: 128s }
    &:nth-child(18) { animation-delay: 136s }
    &:nth-child(19) { animation-delay: 144s }
    &:nth-child(20) { animation-delay: 152s }
    &:nth-child(21) { animation-delay: 160s }
    &:nth-child(22) { animation-delay: 168s }
    &:nth-child(23) { animation-delay: 176s }
    &:nth-child(24) { animation-delay: 184s }
    &:nth-child(25) { animation-delay: 192s }
  }
}
</style>
