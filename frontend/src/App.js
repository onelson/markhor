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
const ZONE_TAB = 1;


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


              if (state.selection.tab !== tabIndex) {
                  store.dispatch(ActionCreators.updateSelectedTab(tabIndex));
              }
          }
          else if (itemKeys.includes(event.keyCode)) {
              // TODO: look at current tab to know which list to alter
              // * move up and down the list of items
              // * interact with an item
              // * handle scrolling (?!?)
              // * (bonus) collapse/expand groups

              if (state.selection.tab === ZONE_TAB) {
                  const zone = state.activeZone;
                  const zoneIds = state.zones.keySeq().toList();

                  // bounds check
                  const first = zoneIds.first();
                  const last = zoneIds.last();

                  let nextZone;

                  if (zone === null && event.keyCode === DOWN) {
                      nextZone = 1;
                  }
                  else if (zone >= first && zone < last && event.keyCode === DOWN) {
                      nextZone = zone + 1;
                  }
                  else if (zone <= last && zone >= first && event.keyCode === UP) {
                      nextZone = zone - 1;
                  }

                  if (nextZone !== undefined) {
                      store.dispatch(ActionCreators.updateActiveZone(nextZone || null));
                  }
              }
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
