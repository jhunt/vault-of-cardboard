<template>
  <div>
    <vcb-goal-list v-if="loaded && goals" :goals="goals"></vcb-goal-list>
    <vcb-loading v-else></vcb-loading>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import VcbLoading from '@/components/loading'
import VcbGoalList from '@/components/goal-list'

export default {
  components: {
    VcbGoalList,
    VcbLoading
  },
  data() {
    return {
      goals: null
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
        fetch('/v1/collectors/'+this.session.uid+'/goals')
          .then(r => r.json())
          .then(those => this.goals = those.goals)
      }
    }
  }
}
</script>
