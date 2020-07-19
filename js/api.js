let authorized = (sid) => new Headers({ 'Authorization': 'Basic '+btoa((sid || '-')+':') });


let fetch_the_json = (url) =>
      fetch(url).then(r => {
        if (!r.ok) {
          throw new Error('API issue trying to get '+url+': got non-ok response '+r);
        }
        return r.json()
      });

let API = {
  load(vault) {
    return Promise.all([
      API.fetch_cards(),
      API.fetch_prices()
    ]).then(data => vault.ingest(data[0], data[1]));
  },

  fetch_cards() {
    return fetch_the_json('/cards.json');
  },

  fetch_prices() {
    return fetch_the_json('/prices.json');
  },

  whoami(sid) {
    return fetch('/v1/whoami', {
      method: 'POST',
      body:   JSON.stringify(sid || '')
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to get /v1/whoami: got non-ok response '+r);
      }
      return r.json()
    });
  },

  signup(username, email, password) {
    return fetch('/v1/signup', {
      method: 'POST',
      body:   JSON.stringify({
        username: username,
        email:    email,
        password: password
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to get /v1/signup: got non-ok response '+r);
      }
      return r.json();
    });
  },

  authenticate(username, password) {
    return fetch('/v1/authenticate', {
      method: 'POST',
      body:   JSON.stringify({
        username: username,
        password: password
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to get /v1/authenticate: got non-ok response '+r);
      }
      return r.json();
    });
  },

  fetch_collection_for(uid) {
    return fetch_the_json('/collectors/'+uid+'/collections/_/collection.json')
      .then(d => { return { base: (d[0] || []), patches: (d[1] || []) } });
  },

  fetch_transactions_for(uid) {
    return fetch_the_json('/v1/collectors/'+uid+'/collections/_/transactions')
      .then(d => d.transactions);
  },

  patch_transaction(auth, txn) {
    let tid = txn.id;
    return fetch('/v1/collectors/'+auth.uid+'/collections/_/transactions/'+tid, {
      method: 'PATCH',
      headers: authorized(auth.session),
      body: JSON.stringify({
        disposition: txn.disposition || '',
        summary:     txn.summary     || '',
        notes:       txn.notes       || '',
        dated:       txn.dated       || '',
        gain:        txn.gain        || '',
        loss:        txn.loss        || '',
        paid:        txn.paid        || null
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to patch /v1/collectors/'+auth.uid+'/collections/_/transactions/'+tid+': got non-ok response '+r);
      }
      return r.json();
    });
  },

  post_transaction(auth, txn) {
    return fetch('/v1/collectors/'+auth.uid+'/collections/_/transactions', {
      method: 'POST',
      headers: authorized(auth.session),
      body: JSON.stringify({
        disposition: txn.disposition || '',
        summary:     txn.summary     || '',
        notes:       txn.notes       || '',
        dated:       txn.dated       || '',
        gain:        txn.gain        || '',
        loss:        txn.loss        || '',
        paid:        txn.paid        || null
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to post /v1/collectors/'+auth.uid+'/collections/_/transactions: got non-ok response '+r);
      }
      return r.json();
    });
  },

  delete_transaction(auth, tid) {
    return fetch('/v1/collectors/'+auth.uid+'/collections/_/transactions/'+tid, {
      method: 'DELETE',
      headers: authorized(auth.session)
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to delete /v1/collectors/'+auth.uid+'/collections/_/transactions/'+tid+': got non-ok response '+r);
      }
      return r.json();
    });
  },

  fetch_goals_for(uid) {
    return fetch_the_json('/v1/collectors/'+uid+'/goals')
      .then(d => d.goals);
  },

  patch_goal(auth, goal) {
    let gid = goal.id;
    return fetch('/v1/collectors/'+auth.uid+'/goals/'+gid, {
      method: 'PATCH',
      headers: authorized(auth.session),
      body: JSON.stringify({
        name:    goal.name    || '',
        target:  goal.target  || '',
        goal:    goal.goal    || ''
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to patch /v1/collectors/'+auth.uid+'/goals/'+gid+': got non-ok response '+r);
      }
      return r.json();
    });
  },

  post_goal(auth, goal) {
    return fetch('/v1/collectors/'+auth.uid+'/goals', {
      method: 'POST',
      headers: authorized(auth.session),
      body: JSON.stringify({
        name:    goal.name    || '',
        target:  goal.target  || '',
        goal:    goal.goal    || '',
        ordinal: goal.ordinal || 0,
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to post /v1/collectors/'+auth.uid+'/goals: got non-ok response '+r);
      }
      return r.json();
    });
  },

  delete_goal(auth, gid) {
    return fetch('/v1/collectors/'+auth.uid+'/goals/'+gid, {
      method: 'DELETE',
      headers: authorized(auth.session)
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to delete /v1/collectors/'+auth.uid+'/goals/'+gid+': got non-ok response '+r);
      }
      return r.json();
    });
  },

  fetch_decks_for(uid) {
    return fetch_the_json('/v1/collectors/'+uid+'/decks')
      .then(d => d.decks);
  },

  patch_deck(auth, deck) {
    let did = deck.id;
    return fetch('/v1/collectors/'+auth.uid+'/decks/'+did, {
      method: 'PATCH',
      headers: authorized(auth.session),
      body: JSON.stringify({
        code:        deck.code        || '',
        title:       deck.title       || '',
        description: deck.description || '',
        main:        deck.main        || '',
        side:        deck.side        || '',
        maybe:       deck.maybe       || ''
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to patch /v1/collectors/'+auth.uid+'/decks/'+did+': got non-ok response '+r);
      }
      return r.json();
    });
  },

  post_deck(auth, deck) {
    return fetch('/v1/collectors/'+auth.uid+'/decks', {
      method: 'POST',
      headers: authorized(auth.session),
      body: JSON.stringify({
        code:        deck.code        || '',
        title:       deck.title       || '',
        description: deck.description || '',
        main:        deck.main        || '',
        side:        deck.side        || '',
        maybe:       deck.maybe       || ''
      })
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to post /v1/collectors/'+auth.uid+'/decks: got non-ok response '+r);
      }
      return r.json();
    });
  },

  delete_deck(auth, did) {
    return fetch('/v1/collectors/'+auth.uid+'/decks/'+did, {
      method: 'DELETE',
      headers: authorized(auth.session)
    }).then(r => {
      if (!r.ok) {
        throw new Error('API issue trying to delete /v1/collectors/'+auth.uid+'/decks/'+did+': got non-ok response '+r);
      }
      return r.json();
    });
  },

};

module.exports.API = API;
