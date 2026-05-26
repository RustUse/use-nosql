use use_nosql::{
    CacheKey, CacheNamespace, ChangeEvent, ChangeEventId, ChangeEventKind, CollectionName,
    ConsistencyLevel, DocumentId, DocumentPath, Embedding, PatchOperation, PatchSet,
    SimilarityMetric, VectorDimension, VectorId, VectorRecord,
};

#[test]
fn facade_reexports_workspace_apis() {
    let collection = CollectionName::new("customers");
    let document_id = DocumentId::new("customer_123");
    let patch = PatchSet::new(vec![PatchOperation::set(
        DocumentPath::new("profile.display_name"),
        "Joshua Whalen",
    )]);

    let cache_key = CacheKey::builder()
        .namespace(CacheNamespace::new("reviews"))
        .segment("google-business-profile")
        .segment("location")
        .segment("fort-wayne")
        .segment("summary")
        .build();

    let vector = VectorRecord::new(
        VectorId::new("review_789_embedding"),
        Embedding::new(vec![0.012, -0.032, 0.481]),
    )
    .with_dimension(VectorDimension::new(3))
    .unwrap()
    .with_similarity_metric(SimilarityMetric::Cosine);

    let event = ChangeEvent::new(
        ChangeEventId::new("evt_1"),
        ChangeEventKind::Update,
        document_id.as_str(),
    );

    assert_eq!(collection.as_str(), "customers");
    assert_eq!(patch.operations().len(), 1);
    assert_eq!(
        cache_key.to_string(),
        "reviews:google-business-profile:location:fort-wayne:summary"
    );
    assert_eq!(vector.dimension(), Some(VectorDimension::new(3)));
    assert_eq!(ConsistencyLevel::LocalQuorum.to_string(), "local-quorum");
    assert_eq!(event.kind(), ChangeEventKind::Update);
}
