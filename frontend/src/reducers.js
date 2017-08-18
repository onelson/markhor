import { combineReducers } from 'redux';
import * as Immutable from 'immutable';

import initialState from './initialState';
import { ActionTypes } from './actions';

function activeZone(state=initialState.activeZone, action) {
    switch(action.type) {
        case ActionTypes.ACTIVE_ZONE_UPDATE:
            return action.zoneId;
        default:
            return state;
    }
}

function dataLoadComplete(state=initialState.dataLoadComplete, action) {
    switch(action.type) {
        case ActionTypes.DATA_LOAD_COMPLETE:
            return true;
        default:
            return state;
    }
}

function categories(state=initialState.categories, action) {
    switch (action.type) {
        case ActionTypes.CATEGORIES_FETCH_SUCCESS:
            return Immutable.Map(action.data.map(x => [x.id, x]));
        case ActionTypes.CATEGORIES_FETCH_FAIL:
            return Immutable.Map();
        default:
            return state;
    }
}
function collectibles(state=initialState.collectibles, action) {
    switch (action.type) {
        case ActionTypes.COLLECTIBLES_FETCH_SUCCESS:
            return Immutable.Map(action.data.map(x => [x.id, x]));
        case ActionTypes.COLLECTIBLES_FETCH_FAIL:
            return Immutable.Map();
        default:
            return state;
    }
}
function membership(state=initialState.membership, action) {
    switch (action.type) {
        case ActionTypes.MEMBERSHIP_FETCH_SUCCESS:
            const list = Immutable.List(action.data);

            const byZone = list
                .groupBy(x => x.zoneId)
                .map(vs => vs.map(v => v.collectibleId));

            const byItem = list
                .groupBy(x => x.collectibleId)
                .map(vs => vs.map(v => v.zoneId));

            return {
                raw: action.data,
                itemsByZone: byZone,
                zonesByItem: byItem
            };
        case ActionTypes.MEMBERSHIP_FETCH_FAIL:
            return initialState.membership;
        default:
            return state;
    }
}
function zones(state=initialState.zones, action) {
    switch (action.type) {
        case ActionTypes.ZONES_FETCH_SUCCESS:
            return Immutable.Map(action.data.map(x => [x.id, x]));
        case ActionTypes.ZONES_FETCH_FAIL:
            return Immutable.Map();
        default:
            return state;
    }
}

export default combineReducers({
    activeZone,
    dataLoadComplete,
    zones,
    collectibles,
    membership,
    categories
});