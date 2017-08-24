import React, { Component } from 'react';


import Provider from 'react-redux/es/components/Provider';


import configureStore from './store/configureStore';
import initialState from './initialState';
import { ActionCreators } from './actions';
import Tabs from './Tabs';

const store = configureStore(initialState);



const LEFT = 37;
const UP = 38;
const RIGHT = 39;
const DOWN = 40;

const tabKeys = [LEFT, RIGHT];
const itemKeys = [UP, DOWN];
const navKeys = tabKeys.concat(itemKeys);



class App extends Component {

  componentDidMount() {
      store.dispatch(ActionCreators.fetchAllData());

      document.body.addEventListener('keydown', (event) => {
          if (!navKeys.includes(event.keyCode)) {
              return;
          }

          const state = store.getState();

          if (tabKeys.includes(event.keyCode)) {
              let tabIndex;
              if (event.keyCode === LEFT) {
                  tabIndex = 0;
              }
              else if (event.keyCode === RIGHT) {
                  tabIndex = 1;
              }


              if (state.view.tab !== tabIndex) {
                  store.dispatch(ActionCreators.updateTabView(tabIndex));
              }
          }

          if (itemKeys.includes(event.keyCode)) {
              // TODO: look at current tab to know which list to alter
              // * move up and down the list of items
              // * interact with an item
              // * handle scrolling (?!?)
              // * (bonus) collapse/expand groups
          }

      });
  }


  render() {
    return (
      <Provider store={store}>
          <Tabs/>
      </Provider>
    );
  }
}

export default App;
