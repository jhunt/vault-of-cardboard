<template>
  <div>
    <vcb-loading v-if="drafted == null"></vcb-loading>
    <template v-else>
      <div class="prose">
        <h2>Draft {{ set }}! <a href="#" @click.prevent="shuffle()" rel="shuffle">↻</a></h2>
      </div>
      <vcb-card-grid :cards="drafted"></vcb-card-grid>
    </template>
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
    set: String
  },
  data() {
    return {
      drafted: null
    }
  },
  computed: {
    ...mapGetters(['loaded', 'vault'])
  },
  mounted() {
    this.vault.on(cardboard.CardsLoaded, () => this.shuffle())
  },
  methods: {
    shuffle() {
      this.drafted = this.vault.draft(this.set).pack()
    }
  }
}
</script>
