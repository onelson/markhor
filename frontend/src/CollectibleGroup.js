import React from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';


CollectibleGroup.propTypes = {
    activeItems: PropTypes.object.isRequired,
    label: PropTypes.string.isRequired,
    items: PropTypes.object.isRequired,
};


// TODO: add local state to control collapsed state.
export default function CollectibleGroup(props) {

    let items = null;

    if (props.items) {
        items = props.items
            .sortBy(x => x.name)
            .map(item => {
            const className = classNames(
                    'item',
                    { inactive: !props.activeItems.includes(item.id)}
                );
            return (
                <li key={item.id}
                    className={className}>
                    {item.name}
                </li>);
        });
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