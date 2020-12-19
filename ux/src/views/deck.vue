<template>
  <div>
    <vcb-deck v-if="loaded && deck" :deck="deck"></vcb-deck>
    <vcb-loading v-else></vcb-loading>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import VcbLoading from '@/components/loading'
import VcbDeck from '@/components/deck'

export default {
  components: {
    VcbDeck,
    VcbLoading
  },
  props: {
    uid: String,
    did: String
  },
  data() {
    return {
      deck: null
    }
  },
  computed: {
    ...mapGetters(['loaded'])
  },
  mounted() {
    this.resync()
  },
  methods: {
    resync() {
      fetch(`/v1/collectors/${this.uid}/decks/${this.did}`)
          .then(r => r.json())
          .then(that => this.deck = that.deck)
    }
  }
}
</script>
