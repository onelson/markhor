import React from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';
import Checkbox from 'semantic-ui-react/dist/commonjs/modules/Checkbox';


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
            <Checkbox label={props.item.name}
                      toggle
                      checked={props.item.collected}
                      onChange={() => props.onToggle(props.item)}/>
        </li>);

}