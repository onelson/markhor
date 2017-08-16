import keyMirror from 'keymirror';

export const ActionTypes = keyMirror({
    // This is meant to be used like a set of enums.
    // Define variants like:
    // `KEY: null,`
});

// A set of functions that return 'action' objects.
// Actions are objects with a `type` field (using a
// value from the `ActionTypes` object, and arbitrary additional
// fields to be used by reducers to perform the requested work.
export const ActionCreators = {};
