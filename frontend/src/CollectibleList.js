import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import * as Immutable from 'immutable';
import { ActionCreators} from './actions';

import CollectibleGroup from './CollectibleGroup';


CollectibleList.propTypes = {
    activeZone: PropTypes.number,
    collectibles: PropTypes.object.isRequired,
    categories: PropTypes.object.isRequired,
    dataLoadComplete: PropTypes.bool.isRequired,
    membership: PropTypes.object.isRequired

};

function mapStateToProps(state) {
    return {
        activeZone: state.activeZone,
        dataLoadComplete: state.dataLoadComplete,
        collectibles: state.collectibles,
        categories: state.categories,
        membership: state.membership
    };
}


function getItemDetails(event) {
    const id = parseInt(event.target.dataset.itemId, 10);
    const value = event.target.checked;
    return { id, value };
}

function CollectibleList(props) {

    if (!props.dataLoadComplete) {
        return null;
    }

    let activeItems;

    if (props.activeZone === null) {
        activeItems = props.collectibles.keySeq();
    }
    else {
        activeItems = props.membership.itemsByZone.get(props.activeZone) || Immutable.List();
    }

    // activeItems = activeItems.toJS();

    const itemsByCategory = props.collectibles
        .valueSeq()
        .groupBy(x => x.category);

    const handleItemToggle = (event) => {
        const { id, value } = getItemDetails(event);
        props.updateCollectible(id, value);
    };

    const groups = props.categories
        .valueSeq()
        .sortBy(x => x.name)  // TODO: downsort groups that are 100% complete!
        .map(category =>
            <CollectibleGroup
                key={category.id}
                activeItems={activeItems}
                label={category.name}
                items={itemsByCategory.get(category.id)}
                onItemToggle={handleItemToggle}
            />
        );

    return (<div className="collectibles">{groups}</div>);
}

export default connect(mapStateToProps, ActionCreators)(CollectibleList);
