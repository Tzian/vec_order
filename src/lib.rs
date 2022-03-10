#[cfg(test)]
mod tests
{
    enum Action
    {
        Push,
        RemoveAt(usize),
        Insert(usize)
    }

    fn get_updates(len: usize, action: Action) -> Vec<(i64, i64)>
    {
        match action
        {
            Action::Push => vec![(-1, (len - 1) as i64)],
            Action::RemoveAt(i) if i == len => vec![(i as i64, -1)],
            Action::RemoveAt(i) =>
            {
                // first we update the removed item
                let mut results: Vec<(i64, i64)> = vec![(i as i64, -1)];

                // how many entries from the one removed to the end of the vec that need to be moved
                let steps = len - i + 1;
                for j in 1..steps
                {
                    let index_to_move = j + i;
                    let new_index = index_to_move - 1;

                    results.push((index_to_move as i64, new_index as i64))
                }
                results
            }
            Action::Insert(i) =>
            {
                // first we update the removed item
                let mut results: Vec<(i64, i64)> = vec![];

                // how many entries from the one removed to the end of the vec that need to be moved
                let steps = len - i;
                for j in 0..steps - 1
                {
                    let index_to_move = i + j;
                    let new_index = index_to_move + 1;

                    results.push((index_to_move as i64, new_index as i64))
                }
                // then update the inserted item
                results.push((-1, i as i64));
                results
            }
        }
    }

    #[test]
    fn push()
    {
        // given vec has length of 4 after the push action, ordering not needed as new item is on end, return (-1 (indicating new item), index of the new item)
        let result = get_updates(4, Action::Push);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (-1, 3));
    }

    #[test]
    fn remove_at_last()
    {
        // given vec has length of 5 after the the remove action
        let length = 5;

        // item removed was last in the vec so its index would be equal to new length
        let result = get_updates(length, Action::RemoveAt(length));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (length as i64, -1));
    }

    #[test]
    fn remove_at()
    {
        // given vec has length of 7 after the the remove action
        let length = 7;
        let removed_index: i64 = 3;
        let result = get_updates(length, Action::RemoveAt(removed_index as usize));

        assert_eq!(result.len(), 5);
        assert_eq!(result[0], (removed_index, -1));
        assert_eq!(result[1], ((removed_index + 1), removed_index));
        assert_eq!(result[2], ((removed_index + 2), (removed_index + 1)));
        assert_eq!(result[3], ((removed_index + 3), (removed_index + 2)));
        assert_eq!(result[4], ((removed_index + 4), (removed_index + 3)));
    }

    #[test]
    fn insert_at()
    {
        // given vec has length of 7 after the the insert action
        let length = 7;

        let inserted_index: i64 = 3;
        let result = get_updates(length, Action::Insert(inserted_index as usize));

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], ((inserted_index), inserted_index + 1));
        assert_eq!(result[1], ((inserted_index + 1), (inserted_index + 2)));
        assert_eq!(result[2], ((inserted_index + 2), (inserted_index + 3)));
        assert_eq!(result[3], (-1, inserted_index));
    }
}
