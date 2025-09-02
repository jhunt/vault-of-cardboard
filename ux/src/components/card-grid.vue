<template>
  <div class="search">
    <div v-if="error" class="oops">{{ query }}: {{ error }}</div>
    <template v-else>
      <div v-if="query" class="meta-results"><div>found <span class="count">{{ found }}</span> card{{ found == 1 ? '' : 's' }}.</div>
        <ul>
          <li v-if="summary.cost > 0.0">${{ dollar$(summary.cost) }} <span>1x</span></li>
          <li v-if="summary.worth > 0.0">${{ dollar$(summary.worth) }} <span>owned</span></li>
          <li>{{ summary.common }} / {{ summary.uncommon }} / {{ summary.rare }} / {{ summary.mythic }}</li>
        </ul>
      </div>
      <div class="results grid">
        <template v-for="(card,i) in visible">
          <vcb-card @click.prevent="showcase = card" :key="i"
                    :backed="!actual && card.owned > 0"
                    :sleeve="sleeve" :card="card"></vcb-card>
          <vcb-card-detail :key="i" :card="showcase" v-if="showcase && card == showcase"
                           @close="showcase = null"></vcb-card-detail>
        </template>
      </div>
      <div class="note meta-results" v-if="more">Not all {{ found }} matching cards displayed.  Have you tried narrowing your search criteria?</div>

    </template>
  </div>
</template>

<script>
import VcbCard from       '@/components/card'
import VcbCardDetail from '@/components/card-detail'

export default {
  name: 'vcb-card-grid',
  components: {
    VcbCard,
    VcbCardDetail
  },
  props: {
    query: String,
    cards: Array,
    sleeve: String,
    actual: Boolean,
    error: String,
    max: {
      type: Number,
      default: 400
    }
  },
  data() {
    return {
      showcase: null
    }
  },
  computed: {
    summary() {
      let summary = {
        cost:     0.0,
        worth:    0.0,
        common:   0,
        uncommon: 0,
        rare:     0,
        mythic:   0,
      };

      this.cards.forEach(card => {
        if (typeof(card.price) !== 'undefined') {
          let price = parseFloat(card.price);
          if (!isNaN(price)) {
            summary.cost  += price;
            summary.worth += price * parseInt(card.owned);
          }
        }
        if (card.flags.indexOf('1') >= 0) { summary.common++; }
        if (card.flags.indexOf('2') >= 0) { summary.uncommon++; }
        if (card.flags.indexOf('3') >= 0) { summary.rare++; }
        if (card.flags.indexOf('4') >= 0) { summary.mythic++; }
      });

      return summary;
    },
    found() {
      return this.cards.length
    },
    visible() {
      return this.cards.slice(0, this.max)
    },
    more() {
      return this.visible.length < this.cards.length
    }
  }
}
</script>

<style lang="scss" scoped>
.meta-results {
  width: 97vw;
  margin: 16px auto;
  color: #777;
  font-style: italic;

  display: flex;
  flex-direction: row;
  justify-content: space-between;

  ul { opacity: 0.1; }
  &:hover ul { opacity: 1; }

  ul {
    display: flex;
    flex-direction: row;

    li {
      font-style: normal;
      font-size: 80%;
      font-family: sans-serif;
      padding-left: 2em;

      span {
        color: #fff;
        background: #9999;
        padding: 3px;
        border-radius: 3px;
        font-size: 90%;
        vertical-align: top;
        font-family: monospace;

      }

      &:hover {
        text-decoration: underline;
        span {
          background-color: #222;
          color: yellow;
        }
      }
    }
  }
}

.grid {
  width: 97vw;
  margin: 8px auto;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-auto-flow: dense;
  grid-gap: 0.68vw;

  @media only screen and (max-width: 759px) {
    grid-template-columns: 1fr;
    grid-gap: 1em;

    .card {
      padding: 0 1em;
    }
  }

  .card-detail {
    grid-column: 1 / 6;
  }
}
</style>
