import * as Immutable from 'immutable';

// This object serves as the initialization
// value for our redux store. Not only does it give us a nice
// convenient place to set defaults, but it also shows us the
// general layout of the data.
export default {
    activeZone: null,
    dataLoadComplete: false,
    zones: Immutable.Map(),
    collectibles: Immutable.Map(),
    membership: {
        raw: [],
        itemsByZone: Immutable.Map(),
        zonesByItem: Immutable.Map()
    },
    categories: Immutable.Map()
};
