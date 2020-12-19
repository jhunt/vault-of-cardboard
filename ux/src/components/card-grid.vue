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
        <vcb-card v-for="(card,i) in visible" :key="i"
                  @click.prevent="showcase = card"
                  :backed="!actual && card.owned > 0"
                  :sleeve="sleeve" :card="card"></vcb-card>
      </div>
      <div class="note meta-results" v-if="more">Not all {{ found }} matching cards displayed.  Have you tried narrowing your search criteria?</div>

      <vcb-card-detail :card="showcase" v-if="showcase"
                       @close="showcase = null"></vcb-card-detail>
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
