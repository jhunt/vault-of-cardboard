<template>
  <div>
    <vcb-card-grid v-if="loaded && results"
                   :error="error" :query="query" :cards="results"></vcb-card-grid>
    <vcb-loading v-else></vcb-loading>
  </div>
</template>

<script>
import cardboard from '@/lib/cardboard/index'
import { mapGetters } from 'vuex'

import VcbCardGrid from '@/components/card-grid'
import VcbLoading  from '@/components/loading'

export default {
  components: {
    VcbCardGrid,
    VcbLoading
  },
  props: {
    query: String
  },
  data() {
    return {
      error:    null,
      results:  null,
    }
  },
  computed: {
    ...mapGetters(['loaded', 'vault'])
  },
  beforeRouteUpdate (to, from, next) {
    this.$store.commit('colorband', null)
    this.search((to ? to.params : this).query)
    next()
  },
  mounted() {
    this.search(this.query)
  },
  methods: {
    search(q) {
      this.$store.dispatch('search', q)
      this.results = null
      this.$store.commit('colorband', null)
      this.vault.on(cardboard.AllLoaded, () => {
        try {
          this.results = this.vault.search(q)
          this.$store.commit('colorband', this.summarize(this.results))
          this.error   = null

        } catch (e) {
          this.results = []
          this.error = e.toString()
        }
      })
    },

    summarize(cards) {
      let colors = {};
      cards.forEach(card => {
        if (card.color == '') {
          colors['C'] = (colors['C'] || 0) + 1;
          colors.total = (colors.total || 0) + 1;
        } else {
          card.color.split('').forEach(color => {
            colors[color] = (colors[color] || 0) + 1;
            colors.total = (colors.total || 0) + 1;
          });
        }
      });

      var at = 0, n = 0, literal;
      var css = 'linear-gradient(90deg';
      'WUBRGC'.split('').forEach(color => {
        if (!(color in colors)) { return; }
        n++;
        css += ', ';
        switch (color) {
        case 'W': literal = '#EEE6B5'; break;
        case 'U': literal = '#375FB6'; break;
        case 'B': literal = '#333333'; break;
        case 'R': literal = '#AE4441'; break;
        case 'G': literal = '#2F7A5B'; break;
        case 'C': literal = '#CBC9C6'; break;
        }
        if (at > 0) {
          css += ' '+at+'vw, ';
        }
        css += literal;
        if (at > 0) {
          css += ' '+at+'vw'
        }
        at += colors[color] / colors.total * 100.0;
      });
      css += ')';
      if (n == 0) { literal = '#CBC9C6'; }
      if (n <  2) { css = literal; }
      return {
        background: css,
        visibility: 'visible',
        height: '7px'
      };
    }
  }
}
</script>
