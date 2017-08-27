import keyMirror from 'keymirror';

export const ActionTypes = keyMirror({
    // This is meant to be used like a set of enums.
    // Define variants like:
    // `KEY: null,`
    ACTIVE_ZONE_UPDATE: null,

    DATA_LOAD_COMPLETE: null,

    CATEGORIES_FETCH_SUCCESS: null,
    CATEGORIES_FETCH_FAIL: null,

    COLLECTIBLES_FETCH_SUCCESS: null,
    COLLECTIBLES_FETCH_FAIL: null,

    MEMBERSHIP_FETCH_SUCCESS: null,
    MEMBERSHIP_FETCH_FAIL: null,

    ZONES_FETCH_SUCCESS: null,
    ZONES_FETCH_FAIL: null,

    COLLECTIBLE_UPDATE_COLLECTED_SUCCESS: null,
    COLLECTIBLE_UPDATE_COLLECTED_FAIL: null,

    SELECTED_TAB_UPDATED: null,
    SELECTED_COLLECTIBLE_UPDATED: null,
    SELECTED_ZONE_UPDATED: null
});

// A set of functions that return 'action' objects.
// Actions are objects with a `type` field (using a
// value from the `ActionTypes` object, and arbitrary additional
// fields to be used by reducers to perform the requested work.

function updateActiveZone(zoneId) {
    return { type: ActionTypes.ACTIVE_ZONE_UPDATE, zoneId };
}

function updateCollectible(collectibleId, value) {
    return (dispatch) => {
        return fetch(
            `/api/collectibles/${collectibleId}/collected/${value}`,
            { method: 'PUT' }
            ).then(
                resp => resp.json(),
                err => {
                    console.error(err);
                    return dispatch({
                        type: ActionTypes.COLLECTIBLE_UPDATE_COLLECTED_FAIL,
                        reason: err
                    });
                }
        ).then( data => dispatch({
            type: ActionTypes.COLLECTIBLE_UPDATE_COLLECTED_SUCCESS,
            data
        }));
    };
}

function fetchCategories() {
    return (dispatch) => {
        return fetch('/api/categories').then(
            resp => resp.json(),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.CATEGORIES_FETCH_FAIL, reason: err });
            }
        ).then(data =>
            dispatch({ type: ActionTypes.CATEGORIES_FETCH_SUCCESS, data }))
    }
}

function fetchCollectibles() {
    return (dispatch) => {
        return fetch('/api/collectibles').then(
            resp => resp.json(),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.COLLECTIBLES_FETCH_FAIL, reason: err });
            }

        ).then(data =>
            dispatch({ type: ActionTypes.COLLECTIBLES_FETCH_SUCCESS, data }))
    }
}

function fetchMembership() {
    return (dispatch) => {
        return fetch('/api/membership').then(
            resp => resp.json(),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.MEMBERSHIP_FETCH_FAIL, reason: err });
            }
        ).then(data =>
            dispatch({ type: ActionTypes.MEMBERSHIP_FETCH_SUCCESS, data }))
    }
}

function fetchZones() {
    return (dispatch) => {
        return fetch('/api/zones').then(
            resp => resp.json(),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.ZONES_FETCH_FAIL, reason: err });
            }
        ).then(data =>
            dispatch({ type: ActionTypes.ZONES_FETCH_SUCCESS, data }))
    }
}

function fetchAllData() {
    return (dispatch) => {
        return Promise.all([
                fetchCategories(),
                fetchCollectibles(),
                fetchMembership(),
                fetchZones(),
            ].map(dispatch)
        ).then(() => dispatch({ type: ActionTypes.DATA_LOAD_COMPLETE }));
    }
}


function updateSelectedTab(index) {
    return { type: ActionTypes.SELECTED_TAB_UPDATED, index };
}

function updateSelectedZone(index) {
    return { type: ActionTypes.SELECTED_ZONE_UPDATED, index };
}

// function updateSelectedCollectible(index) {
//     return { type: ActionTypes.SELECTED_COLLECTIBLE_UPDATED, index };
// }

export const ActionCreators = {
    updateActiveZone,
    updateCollectible,
    fetchAllData,
    fetchCategories,
    fetchCollectibles,
    fetchMembership,
    fetchZones,
    // updateSelectedCollectible,
    updateSelectedTab,
    updateSelectedZone
};
