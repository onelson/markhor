import keyMirror from 'keymirror';

import path from 'path';
import os from 'os';

import { MarkhorDB } from 'markhor-neon';
export const markhorDB = new MarkhorDB(path.resolve(os.homedir(), ".markhordb.sqlite"));

export const ActionTypes = keyMirror({
    // This is meant to be used like a set of enums.
    // Define variants like:
    // `KEY: null,`

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

function updateCollectible(collectibleId, value) {
    return (dispatch) => {
        return new Promise(resolve => {
            resolve(markhorDB.updateCollectibleCollected(collectibleId, value))
        }).then(
                data => dispatch({
                    type: ActionTypes.COLLECTIBLE_UPDATE_COLLECTED_SUCCESS,
                    data
                }),
                err => {
                    console.error(err);
                    return dispatch({
                        type: ActionTypes.COLLECTIBLE_UPDATE_COLLECTED_FAIL,
                        reason: err
                    });
                }
        );
    };
}

function fetchCategories() {
    return (dispatch) => {
        return new Promise(resolve => {
            resolve(markhorDB.getCategories())
        }).then(
            data => dispatch({ type: ActionTypes.CATEGORIES_FETCH_SUCCESS, data }),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.CATEGORIES_FETCH_FAIL, reason: err });
            }
        )
    }
}

function fetchCollectibles() {
    return (dispatch) => {
        return new Promise(resolve => {
            resolve(markhorDB.getCollectibles())
        }).then(
            data => dispatch({ type: ActionTypes.COLLECTIBLES_FETCH_SUCCESS, data }),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.COLLECTIBLES_FETCH_FAIL, reason: err });
            }

        )
    }
}

function fetchMembership() {
    return (dispatch) => {
        return new Promise((resolve) => {
            resolve(markhorDB.getMemberships());
        }).then(
            data => dispatch({ type: ActionTypes.MEMBERSHIP_FETCH_SUCCESS, data }),
            err => {
                console.error(err);
                return dispatch({ type: ActionTypes.MEMBERSHIP_FETCH_FAIL, reason: err });
            }
        )
    }
}

function fetchZones() {
    return (dispatch) => {
        return new Promise((resolve) => {
            resolve(markhorDB.getZones());
        })
        .then(data =>
            dispatch({ type: ActionTypes.ZONES_FETCH_SUCCESS, data }),
                err => {
                    console.error(err);
                    return dispatch({ type: ActionTypes.ZONES_FETCH_FAIL, reason: err });
                })
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

function updateSelectedZone(zoneId) {
    return { type: ActionTypes.SELECTED_ZONE_UPDATED, zoneId };
}

export const ActionCreators = {
    updateCollectible,
    fetchAllData,
    fetchCategories,
    fetchCollectibles,
    fetchMembership,
    fetchZones,
    updateSelectedTab,
    updateSelectedZone
};
