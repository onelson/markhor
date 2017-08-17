import React, { Component } from 'react';
import Provider from 'react-redux/es/components/Provider';
import './App.css';

import configureStore from './store/configureStore';
import initialState from './initialState';
import { ActionCreators } from './actions';

const store = configureStore(initialState);

class App extends Component {

  componentDidMount() {
      store.dispatch(ActionCreators.fetchAllData());
  }
  
  render() {
    return (
      <Provider store={store}>
        <div>
          Hello World!!!!
        </div>
      </Provider>
    );
  }
}

export default App;
