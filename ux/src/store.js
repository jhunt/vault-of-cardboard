import cardboard from '@/lib/cardboard/index'
import Vue from 'vue'
import Vuex from 'vuex'
Vue.use(Vuex)

let vault = new cardboard.Vault()

export default new Vuex.Store({
  state: {
    generation: 1,
    initialized: false,
    query: '',
    colorband: null,
    session: null,
  },

  mutations: {
    increment(state) {
      state.generation++
    },

    uninit(state) {
      state.initialized = false
      state.generation++
    },

    init(state) {
      state.initialized = true
      state.generation++
    },

    search(state, q) {
      state.query = q
      state.generation++
    },

    colorband(state, colorband) {
      state.colorband = colorband
      state.generation++
    },

    session(state, session) {
      if (session) {
        document.cookie = 'vcb_sesh='+session.session+';path=/;max-age=7776000;samesite=strict;'
      } else {
        document.cookie = 'vcb_sesh=;path=/;expires=Thu, 01 Jan 1970 00:00:01 GMT;';
      }
      state.session = session
      state.generation++
    }
  },

  actions: {
    reloadCollection(ctx, auth) {
      vault.unload_collection()
      if (auth) {
        cardboard.API.fetch_collection_for(auth.uid)
          .then(c => vault.on(cardboard.CardsLoaded, () => {
            vault.load_collection(c.base, c.patches)
            ctx.commit('init')
          }))
      } else {
        vault.no_collection()
        ctx.commit('init')
      }
    },

    auth(ctx, auth) {
      ctx.commit('uninit')
      ctx.commit('session', auth)
      ctx.dispatch('reloadCollection', auth)
    },

    load(ctx) {
      var session = (function () {
        var cookies = document.cookie.split(/; */);
        for (var i = 0; i < cookies.length; i++) {
          var kv = cookies[i].split('=');
          if (kv[0] == 'vcb_sesh') {
            return kv[1];
          }
        }
        return undefined;
      })();

      let tasks = [
        fetch('/cards.json').then(r => r.json()),
        fetch('/prices.json').then(r => r.json())
      ]

      if (session) {
        tasks.push(cardboard.API.whoami(session))
      }

      Promise.all(tasks)
        .then(results => {
          let [cards, prices, session] = results
          vault.ingest(cards, prices)

          let auth = session ? session.authenticated : null
          ctx.commit('session', auth)
          ctx.dispatch('reloadCollection', auth)
        })
    },

    search(ctx, q) {
      ctx.commit('search', q)
    }
  },

  getters: {
    query(state) {
      return state.query
    },

    loaded(state) {
      return state.initialized
    },

    session(state) {
      return state.session
    },

    colorband(state) {
      return state.colorband
    },

    vault(state) {
      if (state.generation > 0) {
        return vault
      }
    },

    sets(state) {
      if (state.generation > 0) {
        return vault.sets
      }
    }
  }
})
