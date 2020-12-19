<template>
  <div>
    <vcb-loading v-if="changes == null"></vcb-loading>
    <div class="docs changelog prose" v-else>
      <div v-for="change in changes" :key="change.release">
        <h2 :id="change.release">
          <img :src="'//vaultofcardboard.com'+change.header">
          {{ change.title }}
          <a :href="'#' + change.release">(permalink)</a>
          <span>{{ change.dated }}</span>
        </h2>
        <p v-html="change.notes"></p>
      </div>
    </div>
  </div>
</template>

<script>
import VcbLoading from '@/components/loading'

export default {
  name: 'vcb-changelog',
  components: {VcbLoading},
  data() {
    return {
      changes: null
    }
  },
  mounted() {
    fetch('/changelog.json')
      .then(r => r.json())
      .then(those => this.changes = those.changelog)
  }
}
</script>
