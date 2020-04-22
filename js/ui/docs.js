module.exports = {
  data() {
    return {
      loading: false,
      docs: ''
    };
  },

  created() {
    this.fetch();
  },

  methods: {
    fetch() {
      this.loading = true;
      fetch('/docs.html').then(r => {
        if (!r.ok) { throw new Error('unable to load docs'); }
        return r.text();
      }).then(text => {
        this.docs = text;
        this.loading = false;
      });
    }
  },

  template: `
<loading v-if="loading"></loading>
<div v-else class="docs prose">
  <div v-html="docs"></div>
</div>
`
};
