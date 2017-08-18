import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import * as Immutable from 'immutable';

CollectibleList.propTypes = {
    activeZone: PropTypes.number,
    collectibles: PropTypes.object.isRequired,
    categories: PropTypes.object.isRequired,
    membership: PropTypes.object.isRequired

};

function mapStateToProps(state) {
    return {
        activeZone: state.activeZone,
        collectibles: state.collectibles,
        categories: state.categories,
        membership: state.membership
    };
}

function CollectibleList(props) {
    let items = Immutable.List();

    if (props.activeZone === null) {
        items = props.collectibles.valueSeq();
    }
    else if (props.membership.itemsByZone.has(props.activeZone)) {
        const validIds = props.membership.itemsByZone.get(props.activeZone);
        items = props
            .collectibles
            .valueSeq()
            .filter(x => validIds.includes(x.id)).toList();
    }

    if (items.size > 0) {
        items = items
            .sortBy(x => [x.category, x.name])
            .map(item => <li key={item.id} className="item">{item.name}</li>);
    }
    else {
        items = <li>No items found in this zone.</li>;
    }

    return (
        <ul className="collectible-list">
            {items}
        </ul>);
}

export default connect(mapStateToProps)(CollectibleList);
