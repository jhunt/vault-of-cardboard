<template>
  <div>
    <vcb-loading v-if="!loaded"></vcb-loading>
    <div class="sets prose" v-else>
      <h1>Currently Known Magic: the Gathering Sets</h1>
      <p>These are all the M:tG sets that Vault of Cardboard knows about.
      Most of these should have images.  If a set is missing, or missing its images,
      please let us know!</p>
      <table class="sets sortable listing">
        <thead><tr>
          <th></th>
          <th class="sortable">Code</th>
          <th class="sortable">Name</th>
          <th class="sortable" data-sort-as="number">Release Date</th>
          <th class="sortable" data-sort-as="number">Size</th>
          <th></th>
        </tr></thead>
        <tbody>
          <tr v-for="set in sets"
              :key="set.code"
              :set="set">
            <td><span :class="'ss ss-' + set.code.toLowerCase()"></span></td>
            <td>{{ set.code }}</td>
            <td>{{ set.name }}</td>
            <td :data-sort="set.release">{{ strftime("%B %e, %Y", dated(set.release)) }}</td>
            <td :data-sort="set.size"><router-link :to="{ name: 'search', params: { query: `set:${set.code}` } }">{{ set.size }}</router-link></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import VcbLoading from '@/components/loading'

export default {
  components: {
    VcbLoading
  },
  computed: {
    ...mapGetters(['loaded', 'sets'])
  },
  mounted() {
    //$('#main table.sortable').sortableTable()
  }
}
</script>
