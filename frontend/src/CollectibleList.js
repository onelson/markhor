import React from 'react';
import PropTypes from 'prop-types';
import connect from 'react-redux/es/connect/connect';
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
        activeZone: state.selection.zone,
        dataLoadComplete: state.dataLoadComplete,
        collectibles: state.collectibles,
        categories: state.categories,
        membership: state.membership
    };
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
        activeItems = props.membership
            .itemsByZone.get(props.activeZone) || Immutable.List();
    }

    const itemsByCategory = props.collectibles
        .valueSeq()
        .groupBy(x => x.category);

    const handleItemToggle = (item) => {
        props.updateCollectible(item.id, !item.collected);
    };

    const groups = props.categories
        .valueSeq()
        .map(category => [category, itemsByCategory.get(category.id)])
        .sortBy(([ category, items ]) => {
            return [
                !activeItems.some(x => {
                    const item = items.find(y => y.id === x);
                    return item && !item.collected;
                }),
                category.name ];
        })
        .map(([ category, items ]) =>
            <CollectibleGroup
                key={category.id}
                activeItems={activeItems}
                label={category.name}
                items={items}
                onItemToggle={handleItemToggle}
            />
        );

    return (<div className="collectibles">{groups}</div>);
}

export default connect(mapStateToProps, ActionCreators)(CollectibleList);
