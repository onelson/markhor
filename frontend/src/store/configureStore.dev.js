import { createStore } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';

import enhancer from './enhancer';
import rootReducer from '../reducers';

export default function configureStore(initialState) {
    return createStore(
        rootReducer,
        initialState,
        composeWithDevTools(
            enhancer,
        // other store enhancers if any
        )
    );
}