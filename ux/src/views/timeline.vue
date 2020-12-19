<template>
  <div>
    <vcb-timeline v-if="loaded && transactions" :transactions="transactions"></vcb-timeline>
    <vcb-loading v-else></vcb-loading>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import VcbLoading from '@/components/loading'
import VcbTimeline from '@/components/timeline'

export default {
  components: {
    VcbTimeline,
    VcbLoading
  },
  data() {
    return {
      transactions: null
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
        fetch('/v1/collectors/'+this.session.uid+'/collections/_/transactions')
          .then(r => r.json())
          .then(those => this.transactions = those.transactions)
      }
    }
  }
}
</script>
