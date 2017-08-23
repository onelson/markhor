import React, { Component } from 'react';
import Provider from 'react-redux/es/components/Provider';

import configureStore from './store/configureStore';
import initialState from './initialState';
import { ActionCreators } from './actions';
import ZonePicker from './ZonePicker';
import CollectibleList from './CollectibleList';

const store = configureStore(initialState);

class App extends Component {

  componentDidMount() {
      store.dispatch(ActionCreators.fetchAllData());
  }
  
  render() {
    return (
      <Provider store={store}>
          <div className="app">
              <ZonePicker/>
              <CollectibleList/>
          </div>
      </Provider>
    );
  }
}

export default App;
