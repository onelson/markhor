import { applyMiddleware } from 'redux';
import thunk from 'redux-thunk';

// Middleware for the redux store used in prod/dev (both).
export default applyMiddleware(
    thunk,
    // If more middlewares are needed, just pass
    // them in here as additional args.
    );
