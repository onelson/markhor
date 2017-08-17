import React from 'react';
import PropTypes from 'prop-types';

CollectibleList.propTypes = {
    items: PropTypes.object.isRequired
};

export default function CollectibleList(props) {
    return (<ul>
        <li>items</li>
        <li>items</li>
        <li>items</li>
        <li>items</li>
    </ul>);
}
