import React from 'react';
import PropTypes from 'prop-types';
import List from 'semantic-ui-react/dist/commonjs/elements/List/List';
import CollectibleItem from './CollectibleItem';

CollectibleGroup.propTypes = {
    activeItems: PropTypes.object.isRequired,
    label: PropTypes.string.isRequired,
    items: PropTypes.object.isRequired,
    onItemToggle: PropTypes.func.isRequired
};

export default function CollectibleGroup(props) {
    let items = null;
    if (props.items) {
        items = props.items
            .sortBy(x => [x.shortName, x.name])
            .map(item =>
                <List.Item key={item.id} onClick={() => props.onItemToggle(item)}>
                    <CollectibleItem
                        activeItems={props.activeItems}
                        item={item}/>
                </List.Item>);
    }
    else {
        console.warn(`Group has no items: ${props.label}`);
    }

    const count = (props.items
        ? `${props.items.count(x => x.collected)}/${props.items.size}`
        : '?/?');

    return (
        <div className="collectible-group">
            <h4 className="heading">{props.label} ({count})</h4>
            <List>
                {items}
            </List>
        </div>
    );
}