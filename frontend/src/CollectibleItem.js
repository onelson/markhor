import React from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';


CollectibleItem.propTypes = {
    activeItems: PropTypes.object.isRequired,
    item: PropTypes.object.isRequired,
    onToggle: PropTypes.func.isRequired
};


export default function CollectibleItem(props) {
    const className = classNames(
        'item',
        { inactive: !props.activeItems.includes(props.item.id)}
    );

    return (
        <li className={className}>
            <label htmlFor={`item-${props.item.id}`}>
                {props.item.name}
                <input id={`item-${props.item.id}`}
                       type="checkbox"
                       checked={props.item.collected}
                       data-item-id={props.item.id}
                       onChange={props.onToggle}
                />
            </label>
        </li>);

}