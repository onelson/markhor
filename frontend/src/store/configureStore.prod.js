import { createStore } from 'redux';

import enhancer from './enhancer';
import rootReducer from '../reducers';

export default function configureStore(initialState) {
    // Note: only Redux >= 3.1.0 supports passing enhancer as third argument.
    // See https://github.com/rackt/redux/releases/tag/v3.1.0
    return createStore(rootReducer, initialState, enhancer);
};