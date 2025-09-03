<template>
  <div class="deck prose">
    <vcb-loading v-if="!loaded"></vcb-loading>
    <template v-else-if="mode == 'view'">
      <div class="title">
        <span v-if="deck.code">in:{{ deck.code }}</span>
        <h1>{{ deck.title }}</h1>
      </div>
      <div class="heads-up">
        <div>
          <div v-if="deck.description" v-html="deck.description"></div>
          <template v-else><em>This deck's builder wishes for it to remain a mystery...</em></template>
          <div class="buttons">
            <button class="action" @click.prevent="mode = 'edit'">Edit</button>
            <button class="action" rel="play">Play!</button>
          </div>
        </div>
        <div>
          <div class="showcase">
            <vcb-card v-for="(c,i) in showcase" :sleeve="sleeve" :card="c.card" :key="i"></vcb-card>
          </div>
          <ul :class="'sleeview sl-'+sleeve">
            <li class="sl-w" @click.prevent="sleeve_it('w')">snow</li>
            <li class="sl-u" @click.prevent="sleeve_it('u')">azure</li>
            <li class="sl-b" @click.prevent="sleeve_it('b')">slate</li>
            <li class="sl-r" @click.prevent="sleeve_it('r')">crimson</li>
            <li class="sl-g" @click.prevent="sleeve_it('g')">emerald</li>
          </ul>
        </div>
      </div>

      <h2>Sample Hand <a href="#" @click.prevent="shuffle()" rel="shuffle">↻</a></h2>
      <div class="hand"><vcb-card-grid :sleeve="sleeve" :actual="true" :cards="sample_hand"></vcb-card-grid></div>

      <h2>Next 16 Draws <a href="#" @click.prevent="shuffle()" rel="shuffle">↻</a></h2>
      <div class="draws"><vcb-card-grid :sleeve="sleeve" :actual="true" :cards="next_draws(16)"></vcb-card-grid></div>

      <h2>Mana Curve</h2>
      <div class="curve">
        <div v-for="(stack, cmc) in curve" :key="cmc">
          <ul>
            <li v-for="(card,i) in stack" :key="i">
               <img :src="'/cards/' + card.image"
                    :alt="card.name + ' [' + card.set.code + ']'"
                    :title="card.name + ' [' + card.set.code + ']'"></li>
            <li><img src="/img/mtgback.jpg"></li>
          </ul>
          <p>{{ cmc }}</p>
        </div>
      </div>

      <h2>What's In It?  (Besides {{ cards.length }} card{{ cards.length == 1 ? '' : 's' }})</h2>
      <div class="buttons">
        <button class="action" @click.prevent="mode = 'edit'">Edit</button>
      </div>
      <pre class="rawlist"><code>{{ deck.main }}</code></pre>

    </template>
    <vcb-deck-form
      v-else
      mode="edit"
      @updated-deck="update_deck($event)"
      @deleted-deck="delete_deck($event)"
      :deck="deck"></vcb-deck-form>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import cardboard from '@/lib/cardboard/index'
import _ from '@/lib/helpers'

import VcbCard     from '@/components/card'
import VcbCardGrid from '@/components/card-grid'
import VcbDeckForm from '@/components/deck-form'
import VcbLoading  from '@/components/loading'

export default {
  name: 'vcb-deck',
  components: {
    VcbCard,
    VcbCardGrid,
    VcbDeckForm,
    VcbLoading
  },
  props: ['deck'],
  data: function () {
    return {
      sleeve: 'n',
      shuffled: undefined,
      mode:     'view',
    }
  },
  computed: {
    ...mapGetters(['loaded', 'vault']),
    cards() {
      return this.loaded
        ? this.vault.resolve(this.deck.main)
        : []
    },

    curve() {
      let min = undefined,
          max = 0
      let curve = []

      this.vault.filter(this.cards.map(c => c.card), '!type:land and !type:token').forEach(card => {
        if (!(card.cmc in curve)) {
          curve[card.cmc] = []
        }
        curve[card.cmc].push(card)
        if (card.cmc > max) { max = card.cmc; }
        if (typeof(min) === 'undefined' || card.cmc < min) { min = card.cmc; }
      })

      for (let i = 0; min && i < min; i++) {
        curve[i] = []
      }
      return curve
    },

    showcase() {
      let included = cardboard.Query.parse('!type:land and !type:token')
      let lib = []
      this.cards.forEach(c => {
        if (included.match(c.card)) {
          lib.push(c)
        }
      })
      return lib.slice(0,3)
    },

    library() {
      let included = cardboard.Query.parse('!type:token')
      let lib = []
      this.cards.forEach(c => {
        if (included.match(c.card)) {
          lib.push(c)
        }
      })
      return _.shuffle(lib)
    },

    sample_hand() {
      return this.library.slice(0,7).map(c => c.card)
    },
  },
  methods: {
    next_draws(n) {
      return this.library.slice(8,n+8).map(c => c.card)
    },

    sleeve_it(sl) {
      this.sleeve = sl
    },

    shuffle() {
      this.deck.main = this.deck.main + '\n'
    },

    update_deck(ev) {
      this.deck = ev.deck
      this.mode = 'view'
    },

    delete_deck() {
      this.$router.push({ name: 'decks' })
    }
  }
}
</script>

<style lang="scss">
.deck {
  .hand .grid.results {
    grid-template-columns: repeat(7, 1fr);
  }
  .draws .grid.results {
    grid-template-columns: repeat(8, 1fr);
  }
  .title {
    h1 {
      display: block;
      margin: 4pt 0 1em 0;
    }
    span {
      font-size: 9pt;
      font-family: monospace;
      color: #fff;
      background-color: purple;
      padding: 0.5em;
      border-radius: 0.4em;
    }
  }

  h2 {
    a[rel] {
      border: none;
      color: #aaa;
    }

    &:hover a[rel] {
      color: dodgerblue;
      text-shadow: 0 0 8px lightblue
    }
  }

  .buttons {
    display: flex;
    flex-direction: row;
  }

  .heads-up {
    display: flex;
    flex-direction: row;

    > div:nth-child(2n+1) { width: 48%; }
    > div:nth-child(2n)   { width: 52%; }
  }

  .showcase {
    position: relative;
    border-bottom: 1px solid #ccc;
    min-height: 25vh;
    overflow: hidden;

    .card {
      transform-origin: center bottom;
      position: absolute;
      left: 26%;
      top: 10%;
      max-width: 16vw;

      &:nth-child(1) { transform: rotate(-26deg); }
      &:nth-child(2) { transform: rotate(  0deg); }
      &:nth-child(3) { transform: rotate( 26deg); }
    }
  }

  .sleeview {
    display: flex;
    flex-direction: row;
    justify-content: center;

    li {
      text-indent: -5000px;
      border: 1px solid #111;
      height: 18px;
      width: 18px;
      margin: 12px 6px;

      &.sl-w { background-color: #ece7cb; border-color: #979068; }
      &.sl-u { background-color: #0b80bc; border-color: #243662; }
      &.sl-b { background-color: #444444; border-color: #111111; }
      &.sl-r { background-color: #b22222; border-color: #771515; }
      &.sl-g { background-color: #096e47; border-color: #122f05; }
    }

    li:hover.sl-w, .sl-w .sl-w { box-shadow: 0 0 4px #ece7cb; }
    li:hover.sl-u, .sl-u .sl-u { box-shadow: 0 0 4px #0b80bc; }
    li:hover.sl-b, .sl-b .sl-b { box-shadow: 0 0 4px #444444; }
    li:hover.sl-r, .sl-r .sl-r { box-shadow: 0 0 4px #b22222; }
    li:hover.sl-g, .sl-g .sl-g { box-shadow: 0 0 4px #096e47; }
  }

  .curve {
    display: flex;
    flex-direction: row;
    align-items: flex-end;
    justify-content: center;

    img {
      width: 100%;
      border-radius: 5pt;
      clip-path: url(#card-clip);
    }

    > div {
      max-width: 15vw;
      max-width: 12vw;

      ul {
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        overflow: hidden;
        margin: 0 2pt;

        li {
          height: 2vw;
        }
      }

      p {
        text-align: center;
        font-size: 24pt;
        color: #888;
        border-top: 3px solid #000;
        width: 100%;
        padding-top: 0.5em;
        margin: 0 0 1em 0;
      }
    }
  }

  .grid {
    width: auto;
    &.results > span {
      margin-right: -7em;
    }
    &.results > span:hover {
      z-index: 5;
      transform: scale(120%);
      transition: transform 0.2s;
    }
    .back {
      display: none !important;
    }
  }

  .rawlist {
    font-size: 11pt;
    font-family: monospace;
  }
}

</style>
