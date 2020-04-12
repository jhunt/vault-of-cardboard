let authorized = (sid) => new Headers({ 'Authorization': 'Basic '+btoa((sid || '-')+':') });


let fetch_the_json = (url) =>
      fetch(url).then(r => {
        if (!r.ok) {
          throw new Error('API issue trying to get '+url+': got non-ok response '+r);
        }
        return r.json()
      });

let API = {
  fetch_cards() {
    return fetch_the_json('/cards.json');
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
        summary: txn.summary || '',
        notes:   txn.notes   || '',
        dated:   txn.dated   || '',
        gain:    txn.gain    || '',
        loss:    txn.loss    || ''
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
        summary: txn.summary || '',
        notes:   txn.notes   || '',
        dated:   txn.dated   || '',
        gain:    txn.gain    || '',
        loss:    txn.loss    || ''
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
  }
};

module.exports.API = API;
