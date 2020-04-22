const strftime = require('../helpers.js').strftime;

module.exports = {
  data() {
    return {
      loading: false,
      sets: [],
      sort: { column: '', order:  1 }
    };
  },

  methods: {
    fetch() {
      this.loading = true;
      cardboard.When(cardboard.CardsLoaded, () => {
        this.sets = $vault.sets.map(s => {
          s.release = new Date([... s.release.split(/-/)]);
          s.release_date = strftime("%B %Oe, %Y", s.release);
          return s;
        });
        this.sortBy('release', (a,b) => b.release.getTime() - a.release.getTime());
        this.loading = false;
        $('table.sortable').sortableTable();
      });
    },

    sortBy(c, fn) {
      if (c != this.sort.column) {
        this.sort.column = c;
        this.sort.order = -1;
      }
      this.sort.order *= -1; // flip
      this.sets.sort((a, b) => fn(a,b) * this.sort.order);
    }
  },

  created() {
    this.fetch();
  },

  template: `
<loading v-if="loading"></loading>
<div v-else class="sets prose">
  <h1>Currently Known Magic: the Gathering Sets</h1>
  <p>These are all the M:tG sets that Vault of Cardboard knows about.
  Most of these should have images.  If a set is missing, or missing its images,
  please let us know!</p>
  <table class="sets sortable listing">
    <thead><tr>
      <th></th>
      <th class="sortable" @click="sortBy('code',    (a,b) => a.code.localeCompare(b.code))">Code</th>
      <th class="sortable" @click="sortBy('name',    (a,b) => a.name.localeCompare(b.name))">Name</th>
      <th class="sortable" @click="sortBy('release', (a,b) => b.release.getTime() - a.release.getTime())">Release Date</th>
      <th class="sortable" @click="sortBy('size',    (a,b) => a.size - b.size)">Size</th>
      <th></th>
    </tr></thead>
    <tbody>
      <tr v-for="set in sets"
          :key="set.code"
          :set="set">
        <td><span :class="'ss ss-' + set.code.toLowerCase()"></span></td>
        <td>{{ set.code }}</td>
        <td>{{ set.name }}</td>
        <td>{{ set.release_date }}</td>
        <td><a :href="'#!/q/set:' + set.code">{{ set.size }}</a></td>
      </tr>
    </tbody>
  </table>
</div>
`
};
