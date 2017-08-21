import React from 'react';
import PropTypes from 'prop-types';
import CollectibleItem from './CollectibleItem';


CollectibleGroup.propTypes = {
    activeItems: PropTypes.object.isRequired,
    label: PropTypes.string.isRequired,
    items: PropTypes.object.isRequired,
    onItemToggle: PropTypes.func.isRequired
};


// TODO: add local state to control collapsed state.
export default function CollectibleGroup(props) {

    let items = null;

    if (props.items) {
        items = props.items
            .sortBy(x => x.name)
            .map(item =>
                <CollectibleItem
                    key={item.id}
                    activeItems={props.activeItems}
                    item={item}
                    onToggle={props.onItemToggle}/>);
    }
    else {
        console.warn(`Group has no items: ${props.label}`);
    }

    const count = (props.items
        ? `${props.items.count(x => x.collected)}/${props.items.size}`
        : '?/?');

    return (
        <div>
            <h4>{props.label} ({count})</h4>
            <ul className="collectible-list">
                {items}
            </ul>
        </div>
    );
}