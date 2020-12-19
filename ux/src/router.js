import Vue from 'vue'
import Router from 'vue-router'
Vue.use(Router)

import Changelog from '@/views/changelog'
import Deck      from '@/views/deck'
import Decks     from '@/views/decks'
import Docs      from '@/views/docs'
import Draft     from '@/views/draft'
import DraftSet  from '@/views/draft-set'
import Goals     from '@/views/goals'
import Home      from '@/views/home'
import Search    from '@/views/search'
import Sets      from '@/views/sets'
import Signin    from '@/views/signin'
import Signup    from '@/views/signup'
import Timeline  from '@/views/timeline'

export default new Router({
  mode: 'history',
  base: '/',
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/q/:query',
      name: 'search',
      component: Search,
      props: true
    },
    {
      path: '/signin',
      name: 'signin',
      component: Signin
    },
    {
      path: '/signup',
      name: 'signup',
      component: Signup
    },
    {
      path: '/changelog',
      name: 'changelog',
      component: Changelog
    },
    {
      path: '/sets',
      name: 'sets',
      component: Sets
    },
    {
      path: '/decks',
      name: 'decks',
      component: Decks
    },
    {
      path: '/decks/:uid/:did',
      name: 'deck',
      component: Deck,
      props: true
    },
    {
      path: '/goals',
      name: 'goals',
      component: Goals
    },
    {
      path: '/draft',
      name: 'draft',
      component: Draft
    },
    {
      path: '/draft/:set',
      name: 'draft-set',
      component: DraftSet,
      props: true
    },
    {
      path: '/timeline',
      name: 'timeline',
      component: Timeline
    },
    {
      path: '/docs',
      name: 'docs',
      component: Docs
    }
  ]
})
