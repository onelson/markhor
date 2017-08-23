import React, { Component } from 'react';

import Tab from 'semantic-ui-react/dist/commonjs/modules/Tab';
import Provider from 'react-redux/es/components/Provider';

import configureStore from './store/configureStore';
import initialState from './initialState';
import { ActionCreators } from './actions';
import ZonePicker from './ZonePicker';
import CollectibleList from './CollectibleList';

const store = configureStore(initialState);

const panes = [
    { menuItem: 'Items', render: () => <Tab.Pane><CollectibleList/></Tab.Pane> },
    { menuItem: 'Zones', render: () => <Tab.Pane><ZonePicker/></Tab.Pane> }
];

class App extends Component {

  componentDidMount() {
      store.dispatch(ActionCreators.fetchAllData());
  }
  
  render() {
    return (
      <Provider store={store}>
          <Tab panes={panes} />
      </Provider>
    );
  }
}

export default App;
