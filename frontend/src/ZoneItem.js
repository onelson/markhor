import React from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';

ZoneItem.propTypes = {
    activeZone: PropTypes.any,
    onActivate: PropTypes.func.isRequired,
    item: PropTypes.object.isRequired
};

function ZoneItem(props) {
    const className = classNames(
        "item",
        { active: props.activeZone === props.item.id }
    );

    const handleClick = () => props.onActivate(props.item.id);

    return (
        <li className={className} onClick={handleClick}>
            {props.item.name}
        </li>
    );
}

export default ZoneItem;
