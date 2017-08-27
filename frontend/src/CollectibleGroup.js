import React from 'react';
import PropTypes from 'prop-types';
import Card from 'semantic-ui-react/dist/commonjs/views/Card';
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
            .sortBy(x => x.name)
            .map(item =>
                <List.Item key={item.id}>
                    <CollectibleItem
                        activeItems={props.activeItems}
                        item={item}
                        onToggle={props.onItemToggle}/>
                </List.Item>);
    }
    else {
        console.warn(`Group has no items: ${props.label}`);
    }

    const count = (props.items
        ? `${props.items.count(x => x.collected)}/${props.items.size}`
        : '?/?');

    return (
        <Card fluid>
            <Card.Content>
                <Card.Header>{props.label} ({count})</Card.Header>
            </Card.Content>
            <Card.Content >
            <List>
                {items}
            </List>
            </Card.Content>
        </Card>
    );
}