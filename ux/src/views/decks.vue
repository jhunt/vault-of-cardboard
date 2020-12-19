<template>
  <div>
    <vcb-deck-list v-if="loaded && decks" :decks="decks"></vcb-deck-list>
    <vcb-loading v-else></vcb-loading>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import VcbLoading from '@/components/loading'
import VcbDeckList from '@/components/deck-list'

export default {
  components: {
    VcbDeckList,
    VcbLoading
  },
  data() {
    return {
      decks: null
    }
  },
  computed: {
    ...mapGetters(['loaded', 'session'])
  },
  mounted() {
    this.resync()
  },
  methods: {
    resync() {
      if (this.session) {
        fetch('/v1/collectors/'+this.session.uid+'/decks')
          .then(r => r.json())
          .then(those => this.decks = those.decks)
      }
    }
  }
}
</script>
