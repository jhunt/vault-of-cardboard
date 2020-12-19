import Vue from 'vue'

import Mixin from '@/mixin'
Vue.mixin(Mixin)

import router from '@/router'
import store from '@/store'

import App from '@/App.vue'
new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
