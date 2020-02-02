import React from 'react';
import { Provider } from 'react-redux';
import { hot } from 'react-hot-loader/root';
import App from '../App';

const Root = ({ store }) => (
  <Provider store={store}>
      <App store={store} />
  </Provider>
);

export default hot(Root);
